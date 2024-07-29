use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use axum::routing::get;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use std::error::Error;
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "Service is healthy and running :D")
}

pub async fn home() -> impl IntoResponse {
    (StatusCode::OK, "Home")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "prometheus_metric_example=debug".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/health", get(health))
        .route("/home", get(home))
        .route("/metrics", get(|| async move { metric_handle.render()}))
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Error when creating listener");

    debug!(
        "Listening on {}",
        listener
        .local_addr()
        .expect("Could not convert listener address to local address")
    );

    axum::serve(listener, app)
        .await
        .expect("Error when creating the server");

    Ok(())
}
