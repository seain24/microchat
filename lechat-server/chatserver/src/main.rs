use std::path::PathBuf;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, web};
use clap::Parser;
use chatserver::base::app_state;
use chatserver::error::Result;
use chatserver::{base, interface, Error};
use chatserver::db::Data;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'c', long = "config", value_name = "Config file path")]
    config: PathBuf,
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("welcome to mircr-chat lechat-server!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let cfg = base::config::init_config(&args.config)?;
    let cfg = Arc::new(cfg);
    let cfg_ = cfg.clone();
    let log_cleaner = base::log::init_logger(&cfg)?;
    let addr = format!("{}:{}", cfg.http.ip, cfg.http.port);
    tracing::info!("mirco-chat lechat-server starts running on {}", addr);

    // database connection
    // let conn = base::singleton::db::DbPool::get_instance(cfg_.clone()).await?;
    let db = Arc::new(Data::new(cfg.clone()).await?);
    let app_state = web::Data::new(app_state::AppState::new(cfg_, db.clone()));
    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .max_age(36000);
        App::new()
            .wrap(cors)
            // .app_data(app_state.clone())
            .service(index)
            .configure(|cfg| {
                interface::user::config(cfg);
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
