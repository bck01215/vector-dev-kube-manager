mod kube_agent;
use anyhow::Ok;
use axum::extract::Json as AxumJson;
use axum::{
    extract::State,
    response::sse::{Event, Sse},
    response::Json,
    routing::get,
    routing::post,
    Router,
};
use futures::{stream::Stream, StreamExt};
use std::{path::PathBuf, time::Duration};
use tower_http::services::ServeDir;
use tracing::log::{error, info};
use tracing_subscriber;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct SetDeploymentConfig {
    config: Option<String>,
    message: Option<String>,
    deployment: Option<String>,
    error: Option<String>,
}

#[derive(Clone)]
struct AppState {
    client: kube_agent::KubeClient,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing with log level from RUST_LOG env var
    tracing_subscriber::fmt::init();
    let client = kube_agent::KubeClient::new().await?;

    let state = AppState { client };

    let assets_dir = PathBuf::from(env!("PWD")).join("assets");
    info!("assets dir: {:?}", assets_dir);
    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);
    // build our application with a route
    let app = Router::new()
        .fallback_service(static_files_service)
        .route("/configmaps", get(get_configmaps_handler))
        .route("/set-configmaps", post(set_configmaps_handler))
        .route("/sse", get(sse_handler))
        .with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn get_configmaps_handler(State(state): State<AppState>) -> Json<Vec<String>> {
    let maps = state
        .client
        .get_maps("app.kubernetes.io/name=vector-aws")
        .await
        .unwrap();
    Json(maps)
}

async fn set_configmaps_handler(
    State(state): State<AppState>,
    AxumJson(mut config): AxumJson<SetDeploymentConfig>,
) -> Json<SetDeploymentConfig> {
    let set_config_resp = state
        .client
        .set_deployment(
            "vector-aws",
            &config.clone().config.unwrap_or("no value".to_string()),
            "app.kubernetes.io/name=vector-aws",
        )
        .await;
    if set_config_resp.is_err() {
        config.error = Some(format!("{:?}", set_config_resp.err().unwrap()));
    } else {
        config.message = Some(format!("{:?}", set_config_resp.unwrap()));
    }
    Json(config)
}

async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, anyhow::Error>>> {
    info!("Starting SSE handler");
    let stream = state
        .client
        .watch_pods_stream("app.kubernetes.io/name=vector-aws")
        .await
        .map(|pod| match pod {
            core::result::Result::Ok(pod) => {
                let event = Event::default().data(serde_json::to_string(&pod).unwrap());
                Ok(event)
            }
            Err(err) => {
                error!("Error: {:?}", err);
                Ok(Event::default().data(format!("{:?}", err)))
            }
        });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
