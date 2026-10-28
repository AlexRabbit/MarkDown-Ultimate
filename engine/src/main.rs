use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use mu_diagnostics::{emit, Severity};
use mu_persistence::FileWorkspaceStore;
use mu_workspace::Workspace;
use std::sync::{Arc, Mutex};

struct AppState {
    store: FileWorkspaceStore,
    workspace: Mutex<Workspace>,
}

async fn health() -> HttpResponse {
    emit(
        Severity::Debug,
        "engine",
        "health_check",
        "engine",
        "health ok",
        None,
        None,
    );
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "markdown_ultimate_engine"
    }))
}

async fn get_workspace(data: web::Data<Arc<AppState>>) -> HttpResponse {
    let ws = data.workspace.lock().unwrap().clone();
    HttpResponse::Ok().json(ws)
}

async fn put_workspace(
    data: web::Data<Arc<AppState>>,
    body: web::Json<Workspace>,
) -> HttpResponse {
    let ws = body.into_inner();
    {
        let mut guard = data.workspace.lock().unwrap();
        *guard = ws.clone();
    }
    match data.store.save(&ws) {
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
            HttpResponse::Ok().json(serde_json::json!({"ok": true}))
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
            HttpResponse::InternalServerError().json(serde_json::json!({"ok": false}))
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
        format!("Starting Actix engine on {bind} mode={}", cfg.app_mode),
        None,
        None,
    );
    let state = Arc::new(AppState {
        store,
        workspace: Mutex::new(workspace),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::resource("/api/workspace")
                    .route(web::get().to(get_workspace))
                    .route(web::put().to(put_workspace)),
            )
            .service(web::redirect("/", "/MarkDown-Ultimate/"))
            .service(
                Files::new("/MarkDown-Ultimate", "./dist").index_file("index.html"),
            )
    })
    .bind(&bind)?
    .run()
    .await
}
