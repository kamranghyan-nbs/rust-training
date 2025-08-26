use crate::error::AppError;
use std::env;
use std::io;
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    Layer, Registry,
};

pub struct LoggingConfig {
    pub level: String,
    pub format: LogFormat,
    pub output: LogOutput,
    pub file_config: Option<FileConfig>,
    pub enable_tokio_console: bool,
    pub enable_distributed_tracing: bool,
    pub sentry_dsn: Option<String>,
}

#[derive(Debug, Clone)]
pub enum LogFormat {
    Pretty,  // Human-readable for development
    Json,    // Structured JSON for production
    Compact, // Compact format for resource-constrained environments
}

#[derive(Debug, Clone)]
pub enum LogOutput {
    Console,
    File,
    Both,
}

#[derive(Debug, Clone)]
pub struct FileConfig {
    pub directory: String,
    pub file_name_prefix: String,
    pub rotation: FileRotation,
}

#[derive(Debug, Clone)]
pub enum FileRotation {
    Hourly,
    Daily,
    Never,
}

impl LoggingConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        let format = match env::var("LOG_FORMAT").as_deref() {
            Ok("json") => LogFormat::Json,
            Ok("compact") => LogFormat::Compact,
            _ => LogFormat::Pretty,
        };

        let output = match env::var("LOG_OUTPUT").as_deref() {
            Ok("file") => LogOutput::File,
            Ok("both") => LogOutput::Both,
            _ => LogOutput::Console,
        };

        let file_config = if matches!(output, LogOutput::File | LogOutput::Both) {
            Some(FileConfig {
                directory: env::var("LOG_DIR").unwrap_or_else(|_| "./logs".to_string()),
                file_name_prefix: env::var("LOG_FILE_PREFIX")
                    .unwrap_or_else(|_| "product-api".to_string()),
                rotation: match env::var("LOG_ROTATION").as_deref() {
                    Ok("hourly") => FileRotation::Hourly,
                    Ok("never") => FileRotation::Never,
                    _ => FileRotation::Daily,
                },
            })
        } else {
            None
        };

        let enable_tokio_console = env::var("ENABLE_TOKIO_CONSOLE")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(false);

        let enable_distributed_tracing = env::var("ENABLE_DISTRIBUTED_TRACING")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(false);

        let sentry_dsn = env::var("SENTRY_DSN").ok();

        Ok(LoggingConfig {
            level,
            format,
            output,
            file_config,
            enable_tokio_console,
            enable_distributed_tracing,
            sentry_dsn,
        })
    }
}

// Guards to keep async writers alive
pub struct LoggingGuards {
    _console_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
    _file_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

pub async fn init_async_logging() -> Result<LoggingGuards, AppError> {
    let config = LoggingConfig::from_env()?;

    info!("Initializing async logging system...");
    info!("Log level: {}", config.level);
    info!("Log format: {:?}", config.format);
    info!("Log output: {:?}", config.output);

    // Create the base registry
    // let registry = Registry::default();

    // Initialize guards to keep async writers alive
    let mut console_guard = None;
    let mut file_guard = None;

    // Create env filter
    // let env_filter = EnvFilter::try_from_default_env()
    //     .or_else(|_| EnvFilter::try_new(&config.level))
    //     .map_err(|e| AppError::BadRequest {
    //         message: format!("Invalid log level: {}", e),
    //         error_id: uuid::Uuid::new_v4(),
    //     })?;

    // Build the subscriber with different layers
    let mut layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = Vec::new();

    // Console Layer (if needed)
    if matches!(config.output, LogOutput::Console | LogOutput::Both) {
        let (console_writer, guard) = non_blocking(io::stdout());
        console_guard = Some(guard);

        let console_layer = match config.format {
            LogFormat::Pretty => fmt::layer()
                .with_writer(console_writer)
                .with_ansi(true)
                .with_span_events(FmtSpan::CLOSE)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .boxed(),

            LogFormat::Json => fmt::layer()
                .with_writer(console_writer)
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .boxed(),

            LogFormat::Compact => fmt::layer()
                .with_writer(console_writer)
                .compact()
                .with_ansi(false)
                .boxed(),
        };

        layers.push(console_layer);
    }

    // File Layer (if needed)
    if let Some(file_config) = &config.file_config {
        let file_appender = match file_config.rotation {
            FileRotation::Hourly => {
                rolling::hourly(&file_config.directory, &file_config.file_name_prefix)
            }
            FileRotation::Daily => {
                rolling::daily(&file_config.directory, &file_config.file_name_prefix)
            }
            FileRotation::Never => rolling::never(
                &file_config.directory,
                format!("{}.log", file_config.file_name_prefix),
            ),
        };

        let (file_writer, guard) = non_blocking(file_appender);
        file_guard = Some(guard);

        // Always use JSON format for file logging in production
        let file_layer = fmt::layer()
            .with_writer(file_writer)
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .boxed();

        layers.push(file_layer);

        info!(
            "File logging enabled: {}/{}",
            file_config.directory, file_config.file_name_prefix
        );
    }

    // Sentry Layer (for error reporting)
    if let Some(sentry_dsn) = &config.sentry_dsn {
        let _guard = sentry::init((
            sentry_dsn.as_str(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                environment: Some(
                    env::var("ENVIRONMENT")
                        .unwrap_or_else(|_| "development".into())
                        .into(),
                ),
                ..Default::default()
            },
        ));

        let sentry_layer = sentry_tracing::layer().boxed();
        layers.push(sentry_layer);

        info!("Sentry error reporting enabled");
    }


    info!("Async logging system initialized successfully");

    Ok(LoggingGuards {
        _console_guard: console_guard,
        _file_guard: file_guard,
    })
}

// Structured logging macros for better consistency
#[macro_export]
macro_rules! log_request {
    ($method:expr, $path:expr, $user_id:expr) => {
        tracing::info!(
            method = %$method,
            path = %$path,
            user_id = %$user_id,
            "API request received"
        );
    };
}

#[macro_export]
macro_rules! log_response {
    ($method:expr, $path:expr, $status:expr, $duration_ms:expr) => {
        tracing::info!(
            method = %$method,
            path = %$path,
            status = %$status,
            duration_ms = %$duration_ms,
            "API request completed"
        );
    };
}

#[macro_export]
macro_rules! log_error {
    ($error:expr, $context:expr) => {
        tracing::error!(
            error = %$error,
            context = %$context,
            "Application error occurred"
        );
    };
}

#[macro_export]
macro_rules! log_database_query {
    ($query_type:expr, $table:expr, $duration_ms:expr) => {
        tracing::debug!(
            query_type = %$query_type,
            table = %$table,
            duration_ms = %$duration_ms,
            "Database query executed"
        );
    };
}

// Performance monitoring utilities
// pub fn create_span(name: &'static str) -> tracing::Span {
//     tracing::info_span!(name)
// }

// pub fn create_span_with_fields(name: &'static str, fields: &[(&str, &dyn std::fmt::Display)]) -> tracing::Span {
//     let span = tracing::info_span!(name);
//     for (key, value) in fields {
//         span.record(key, &tracing::field::display(value));
//     }
//     span
// }
