use std::path::PathBuf;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use clap::Parser;
use micro_chat_server::base::app_state;
use micro_chat_server::error::Result;
use micro_chat_server::{base, transport, Error};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'c', long = "config", value_name = "Config file path")]
    config: PathBuf,
}

#[get("/")]
async fn init() -> HttpResponse {
    HttpResponse::Ok().body("welcome to mircr-chat server!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let cfg = base::cfg::init_config(&args.config)?;
    let cfg = Arc::new(cfg);
    let cfg_ = cfg.clone();
    let log_cleaner = base::log::init_logger(&cfg)?;
    let addr = format!("{}:{}", cfg.http.ip, cfg.http.port);
    tracing::info!("mirco-chat server starts running on {}", addr);

    // database connection
    let conn = base::singleton::db::DbPool::get_instance(cfg_.clone()).await?;
    let app_state = web::Data::new(app_state::AppState::new(cfg_, conn));
    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .max_age(36000);
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(init)
            .configure(|cfg| {
                transport::api::register(cfg);
            })
    })
    .workers(4)
    .shutdown_timeout(5)
    .bind(addr)
    .map_err(|e| Error::ServerError(e.to_string()))?
    .run()
    .await;

    log_cleaner();
    Ok(())
}
