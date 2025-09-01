use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub struct LoggingConfig {
    pub level: String,
    pub format: LogFormat,
    pub service_name: String,
    pub jaeger_endpoint: Option<String>,
}

#[derive(Debug, Clone)]
pub enum LogFormat {
    Json,
    Pretty,
}

impl From<&str> for LogFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => LogFormat::Json,
            "pretty" => LogFormat::Pretty,
            _ => LogFormat::Pretty,
        }
    }
}

// Change the return type to use anyhow::Result
pub fn init_tracing(config: LoggingConfig) -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));

    let registry = tracing_subscriber::registry().with(env_filter);

    match config.format {
        LogFormat::Json => {
            registry
                .with(fmt::layer().json().with_span_events(fmt::format::FmtSpan::CLOSE))
                .try_init()?;
        }
        LogFormat::Pretty => {
            registry
                .with(fmt::layer().pretty().with_span_events(fmt::format::FmtSpan::CLOSE))
                .try_init()?;
        }
    }

    Ok(())
}

// Middleware for request tracing
pub mod middleware {
    use axum::http::Request;
    use std::time::Instant;
    use tower::{Layer, Service};
    use tracing::{info_span, Instrument};
    use uuid::Uuid;
    use std::task::{Context, Poll};
    use std::pin::Pin;
    use std::future::Future;

    #[derive(Clone)]
    pub struct RequestTracingLayer;

    impl<S> Layer<S> for RequestTracingLayer {
        type Service = RequestTracingService<S>;

        fn layer(&self, inner: S) -> Self::Service {
            RequestTracingService { inner }
        }
    }

    #[derive(Clone)]
    pub struct RequestTracingService<S> {
        inner: S,
    }

    impl<S, B> Service<Request<B>> for RequestTracingService<S>
    where
        S: Service<Request<B>>,
        S::Future: Send + 'static,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, req: Request<B>) -> Self::Future {
            let request_id = Uuid::new_v4();
            let method = req.method().clone();
            let uri = req.uri().clone();
            let start_time = Instant::now();
            
            let span = info_span!(
                "request",
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status_code = tracing::field::Empty,
                duration_ms = tracing::field::Empty,
            );

            let future = self.inner.call(req);
            
            Box::pin(async move {
                let result = future.await;
                let duration = start_time.elapsed();
                
                tracing::info!(
                    duration_ms = duration.as_millis(),
                    "Request completed in {}ms",
                    duration.as_millis()
                );
                
                result
            }.instrument(span))
        }
    }
}