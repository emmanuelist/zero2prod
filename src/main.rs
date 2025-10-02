use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test, web};

    #[tokio::test]
    async fn health_check_succeeds() {
        //create test app
        let app =
            test::init_service(App::new().route("/health_check", web::get().to(health_check)))
                .await;

        //create test request
        let req = test::TestRequest::get().uri("/health_check").to_request();

        //Perform the request and verify the response
        let response = test::call_service(&app, req).await;

        assert!(response.status().is_success())
    }
}
