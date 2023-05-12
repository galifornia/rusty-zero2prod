use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn check_health(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(check_health)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}
