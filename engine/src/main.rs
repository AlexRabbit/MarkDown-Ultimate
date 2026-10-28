use actix_files::Files;
use actix_web::{get, http::header, put, web, App, HttpResponse, HttpServer, Responder};
use mu_diagnostics::{emit, Severity};
use mu_persistence::FileWorkspaceStore;
use mu_workspace::Workspace;
use std::sync::Mutex;

struct AppState {
    store: FileWorkspaceStore,
    workspace: Mutex<Workspace>,
}

#[get("/")]
async fn root_redirect() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "/MarkDown-Ultimate/"))
        .finish()
}

#[get("/health")]
async fn health() -> impl Responder {
    emit(
        Severity::Debug,
        "engine",
        "health_check",
        "engine",
        "health ok",
        None,
        None,
    );
    HttpResponse::Ok().json(serde_json::json!({"status":"ok","service":"markdown_ultimate_engine"}))
}

#[get("/api/workspace")]
async fn get_workspace(state: web::Data<AppState>) -> impl Responder {
    let ws = state.workspace.lock().unwrap().clone();
    HttpResponse::Ok().json(ws)
}

#[put("/api/workspace")]
async fn put_workspace(body: web::Json<Workspace>, state: web::Data<AppState>) -> impl Responder {
    let ws = body.into_inner();
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
        format!("Starting engine on {bind} mode={}", cfg.app_mode),
        None,
        None,
    );
    let state = web::Data::new(AppState {
        store,
        workspace: Mutex::new(workspace),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(root_redirect)
            .service(health)
            .service(get_workspace)
            .service(put_workspace)
            .service(
                Files::new("/MarkDown-Ultimate", "./dist").index_file("index.html"),
            )
    })
    .bind(bind)?
    .run()
    .await
}
