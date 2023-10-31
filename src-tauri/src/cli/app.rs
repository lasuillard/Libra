use anyhow::Result;
use opentelemetry::{global, logs::LogError, metrics, trace::TraceError};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{metrics::MeterProvider, runtime, trace as sdktrace};
use tracing_subscriber::prelude::*;

use crate::cli::common::arg_env;

/// Start GUI application.
#[derive(clap::Args)]
pub(super) struct Args {
    /// Application log level.
    #[clap(long, required = false, default_value_t = log::Level::Warn, env = arg_env!("LOG_LEVEL"))]
    pub(super) log_level: log::Level,
}

// NOTE: Don't forget to make sync with source struct
impl Default for Args {
    fn default() -> Self {
        Self {
            log_level: log::Level::Warn,
        }
    }
}

/// Start application with pre and post app tasks.
pub(super) async fn execute(_cli: &super::Cli, args: &Args) -> Result<()> {
    // Init tracing
    global::set_text_map_propagator(opentelemetry_sdk::propagation::TraceContextPropagator::new());
    let tracer = init_tracer()?;
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    // Init logging
    let _ = init_logs()?;
    let otel_log_appender = OpenTelemetryLogBridge::new(&global::logger_provider());
    multi_log::MultiLogger::init(
        vec![
            simplelog::TermLogger::new(
                args.log_level.to_level_filter(),
                simplelog::Config::default(),
                simplelog::TerminalMode::Stdout,
                simplelog::ColorChoice::Auto,
            ),
            Box::new(otel_log_appender),
        ],
        args.log_level,
    )?;

    // Init metrics
    let meter_provider = init_metrics()?;

    // Run GUI application
    crate::run();

    // Graceful shutdown
    global::shutdown_tracer_provider();
    global::shutdown_logger_provider();
    meter_provider.shutdown()?;

    Ok(())
}

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .install_batch(runtime::Tokio)
}

fn init_metrics() -> metrics::Result<MeterProvider> {
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .build()
}

fn init_logs() -> Result<opentelemetry_sdk::logs::Logger, LogError> {
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .install_batch(runtime::Tokio)
}
