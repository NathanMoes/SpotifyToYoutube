use std::env;
use tracing::{Level, Span};
use tracing_subscriber::{
    filter::EnvFilter,
    fmt::{self, time::ChronoUtc},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};
use tower_http::trace::TraceLayer;
use axum::http::{Request, Response};
use std::time::Duration;

/// Initialize comprehensive logging and tracing
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Get log level from environment variable, default to INFO
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info,spotify_to_youtube_backend=debug".to_string());
    
    // Create environment filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    // Configure JSON logging for production
    let json_layer = if env::var("LOG_FORMAT").unwrap_or_default() == "json" {
        Some(
            fmt::layer()
                .json()
                .with_timer(ChronoUtc::rfc_3339())
                .with_target(true)
                .with_level(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_filter(env_filter)
        )
    } else {
        None
    };

    // Configure human-readable logging for development
    let fmt_layer = if json_layer.is_none() {
        Some(
            fmt::layer()
                .with_timer(ChronoUtc::rfc_3339())
                .with_target(true)
                .with_level(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_ansi(true)
        )
    } else {
        None
    };

    // Initialize the subscriber
    let registry = tracing_subscriber::registry();
    
    if let Some(json) = json_layer {
        registry.with(json).init();
    } else if let Some(fmt) = fmt_layer {
        registry.with(fmt).init();
    } else {
        // Fallback to basic logging
        tracing_subscriber::fmt::init();
    }

    tracing::info!("Logging initialized successfully");
    Ok(())
}

/// Create HTTP tracing layer with request/response logging
pub fn create_trace_layer() -> TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    impl tower_http::trace::MakeSpan<axum::body::Body> + Clone,
    impl tower_http::trace::OnRequest<axum::body::Body> + Clone,
    impl tower_http::trace::OnResponse<axum::body::Body> + Clone
> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<axum::body::Body>| {
            let uri = request.uri().to_string();
            let method = request.method().to_string();
            let user_agent = request
                .headers()
                .get("user-agent")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown");
            
            tracing::info_span!(
                "http_request",
                method = %method,
                uri = %uri,
                user_agent = %user_agent,
                request_id = %uuid::Uuid::new_v4(),
            )
        })
        .on_request(|request: &Request<axum::body::Body>, _span: &Span| {
            tracing::info!(
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version(),
                "HTTP request started"
            );
        })
        .on_response(|response: &Response<axum::body::Body>, latency: Duration, _span: &Span| {
            let status = response.status();
            let latency_ms = latency.as_millis();
            
            let _log_level = if status.is_server_error() {
                Level::ERROR
            } else if status.is_client_error() {
                Level::WARN
            } else {
                Level::INFO
            };

            tracing::event!(
                target: "spotify_to_youtube_backend",
                Level::INFO,
                status = %status,
                latency_ms = %latency_ms,
                "HTTP request completed"
            );

            // Log slow requests
            if latency_ms > 1000 {
                tracing::warn!(
                    latency_ms = %latency_ms,
                    status = %status,
                    "Slow request detected"
                );
            }
        })
}

/// Log application startup information
pub fn log_startup_info(port: &str) {
    tracing::info!(
        port = %port,
        app_version = env!("CARGO_PKG_VERSION"),
        "Application starting"
    );
}

/// Log application shutdown
pub fn log_shutdown() {
    tracing::info!("Application shutting down");
}

/// Utility macro for timing operations
#[macro_export]
macro_rules! time_operation {
    ($operation:expr, $($field:tt)*) => {{
        let start = std::time::Instant::now();
        let result = $operation;
        let duration = start.elapsed();
        
        tracing::info!(
            duration_ms = %duration.as_millis(),
            $($field)*,
            "Operation completed"
        );
        
        result
    }};
}

/// Utility macro for error context
#[macro_export]
macro_rules! log_error_context {
    ($error:expr, $context:expr) => {
        tracing::error!(
            error = %$error,
            context = %$context,
            "Operation failed"
        );
    };
    ($error:expr, $context:expr, $($field:tt)*) => {
        tracing::error!(
            error = %$error,
            context = %$context,
            $($field)*,
            "Operation failed"
        );
    };
}
