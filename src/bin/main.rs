use axum::extract::Extension;
use axum::routing::get;
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

use rust_sample_app::routes::create_person;
use rust_sample_app::routes::fast;
use rust_sample_app::routes::health_check;
use rust_sample_app::routes::list_persons;
use rust_sample_app::routes::metrics;
use rust_sample_app::routes::slow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".to_owned());

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(trace::config().with_resource(Resource::new([KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "rust-sample-app",
        )])))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&otel_endpoint),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    //let logger = tracing_subscriber::fmt::layer();

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let bun = tracing_bunyan_formatter::BunyanFormattingLayer::new(
        "rust-sample-app".into(),
        std::io::stdout,
    );

    let collector = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(bun);

    tracing::subscriber::set_global_default(collector).unwrap();

    let database = sled::open("/tmp/rust-sample-app").unwrap();

    let app = axum::Router::new()
        .route("/healthz/", get(health_check))
        .route("/metrics/", get(metrics))
        .route("/v1/people/", get(list_persons).post(create_person))
        .route("/v1/fast/", get(fast))
        .route("/v1/slow/", get(slow))
        .layer(Extension(database));

    let socket = (
        [0, 0, 0, 0],
        std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_owned())
            .parse()
            .unwrap(),
    );
    let socket = std::net::SocketAddr::from(socket);

    info!("server is starting on {:?}", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
