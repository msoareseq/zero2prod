async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}