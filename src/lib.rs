use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
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
            .route("/{name}", web::get().to(greet))
    })
    // .bind(listener)?
    .listen(listener)?
    .run();
    // .await
    Ok(server)
}
