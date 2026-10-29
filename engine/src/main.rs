use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use mu_diagnostics::{emit, Severity};
use mu_persistence::FileWorkspaceStore;
use mu_workspace::Workspace;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

struct AppState {
    store: FileWorkspaceStore,
    workspace: Mutex<Workspace>,
}

async fn health() -> impl IntoResponse {
    emit(
        Severity::Debug,
        "engine",
        "health_check",
        "engine",
        "health ok",
        None,
        None,
    );
    Json(serde_json::json!({"status":"ok","service":"markdown_ultimate_engine"}))
}

async fn get_workspace(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let ws = state.workspace.lock().unwrap().clone();
    Json(ws)
}

async fn put_workspace(
    State(state): State<Arc<AppState>>,
    Json(ws): Json<Workspace>,
) -> impl IntoResponse {
    {
        let mut guard = state.workspace.lock().unwrap();
        *guard = ws.clone();
    }
    match state.store.save(&ws) {
        Ok(()) => {
            emit(
                Severity::Info,
                "engine",
                "workspace_saved",
                "engine",
                "workspace persisted",
                None,
                None,
            );
            publish_nats_event("mdu.workspace.updated");
            (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response()
        }
        Err(e) => {
            emit(
                Severity::Error,
                "engine",
                "workspace_save_failed",
                "engine",
                e.to_string(),
                None,
                None,
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"ok": false})),
            )
                .into_response()
        }
    }
}

fn publish_nats_event(subject: &str) {
    let url = std::env::var("NATS_URL").unwrap_or_default();
    if url.is_empty() {
        return;
    }
    let subject = subject.to_string();
    std::thread::spawn(move || {
        if let Ok(rt) = tokio::runtime::Runtime::new() {
            let _ = rt.block_on(async {
                if let Ok(client) = async_nats::connect(url).await {
                    let _ = client.publish(subject.to_string(), "1".into()).await;
                }
            });
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = mu_config::load_from_env_file(".env").unwrap_or_default();
    std::env::set_var("LOGS_DIR", &cfg.logs_dir);
    let bind = cfg.engine_bind.clone();
    let store = FileWorkspaceStore::new("databases/data");
    let workspace = store.load().unwrap_or_default();
    emit(
        Severity::Info,
        "engine",
        "startup",
        "engine",
        format!("Starting engine on {bind} mode={}", cfg.app_mode),
        None,
        None,
    );
    let state = Arc::new(AppState {
        store,
        workspace: Mutex::new(workspace),
    });

    let app = Router::new()
        .route("/health", get(health))
        .route(
            "/api/workspace",
            get(get_workspace).put(put_workspace),
        )
        .route("/", get(|| async { Redirect::temporary("/MarkDown-Ultimate/") }))
        .nest_service(
            "/MarkDown-Ultimate",
            ServeDir::new("./dist").append_index_html_on_directories(true),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
