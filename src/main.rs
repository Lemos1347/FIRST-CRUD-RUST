mod api;
mod db;

use api::task::{create_user, delete_user_by_id, edit_user_by_id, get_user_by_email, get_users, get_user_by_id};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    // Creating tables and syncing db
    let client = Data::new(db::new_client().await.unwrap());
    #[cfg(debug_assertions)]
    client._db_push().await.unwrap();

    // Hosting server
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(client.clone())
            .service(create_user)
            .service(get_users)
            .service(get_user_by_email)
            .service(delete_user_by_id)
            .service(edit_user_by_id)
            .service(get_user_by_id)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
