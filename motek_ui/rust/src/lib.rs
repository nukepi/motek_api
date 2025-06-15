mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod api;
pub mod api_handlers; 
pub mod models;
pub mod utils;

use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use std::sync::Once;

static INIT: Once = Once::new();


pub fn init_logging() {
    INIT.call_once(|| {
        let log_path = std::env::temp_dir().join("motek_ui.log");
        let log_path_str = log_path.to_string_lossy();
        
        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            std::env::temp_dir(),
            "motek_ui.log",
        );

        let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());

        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(EnvFilter::from_default_env()
                .add_directive("motek_ui=trace".parse().unwrap())
                .add_directive("info".parse().unwrap()))
            .with_writer(file_appender)
            .with_writer(non_blocking) // Dodanie logowania do konsoli
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");

        tracing::info!("Logging initialized with detailed configuration");
        tracing::info!("Log file location: {}", log_path_str);
        
        if let Ok(api_url) = std::env::var("API_URL") {
            tracing::info!("API_URL from environment: {}", api_url);
        } else {
            tracing::info!("API_URL not set in environment, will use default");
        }
    });
}

