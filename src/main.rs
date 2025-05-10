mod controllers;
mod adapter;
mod domain;

use actix_web::{web, App, HttpServer};
use controllers::receive_user_extract_controller::ReceiveUserExtractController;
use domain::service::process_files_service::ProcessFilesService;
use adapter::receive_user_extract_adapter::ReceiveUserExtractAdapter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let process_files_service = ProcessFilesService::new();
    let adapter = ReceiveUserExtractAdapter::new(process_files_service);
    let controller = web::Data::new(ReceiveUserExtractController::new(adapter));

    HttpServer::new(move || {
        App::new()
            .app_data(controller.clone())
            .service(
                web::scope("/api")
                    .service(controllers::receive_user_extract_controller::process_files)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} 