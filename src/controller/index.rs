use actix_web::web::Json;
use actix_web::get;

#[get("/")]
pub async fn index() -> Json<&'static str> {
    Json("sss")
}