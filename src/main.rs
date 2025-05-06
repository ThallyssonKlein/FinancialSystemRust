mod controllers;

use actix_web::{web, App, HttpServer};
use controllers::receive_user_extract_controller::ReceiveUserExtractController;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let controller = web::Data::new(ReceiveUserExtractController::new());

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