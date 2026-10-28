use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mu_diagnostics::{emit, Severity};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = mu_config::load_from_env_file(".env").unwrap_or_default();
    let bind = std::env::var("ENGINE_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into());
    emit(
        Severity::Info,
        "engine",
        "startup",
        "engine",
        format!("Starting engine on {bind} mode={}", cfg.app_mode),
        None,
        None,
    );
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .bind(bind)?
    .run()
    .await
}
