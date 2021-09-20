//! src/startup.rs

use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new (move || {
        App::new()
            // Route combines a handler with a set of guards.
            // Guards specify conditions that a request must satisfy in order
            // to "match" and be passed over to the handler.
            // From an implementation standpoint guards are implementors of
            // the Guard trait: Guard::check is where the magic happens.
            // web::get() is a short-cut for Route::new().guard(guard::Get())
            // a.k.a. the request should be passed to the handler if and only
            // if its HTTP method is GET.
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}