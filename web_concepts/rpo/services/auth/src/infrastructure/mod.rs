pub mod database;
pub mod entities;
pub mod repositories;
pub mod migrations;  // Add this line

pub use database::*;
pub use migrations::*;  // Add this line