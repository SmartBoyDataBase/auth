use actix::Addr;
use actix_web::{HttpResponse, Responder, web};
use actix_web::body::Body;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::db::{LoginRequest, PgConnection, UserRoleRequest};
use crate::service::generate_token;

pub async fn ping() -> impl Responder {
    "pong"
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    token: String,
    role: i64,
}

pub async fn login(db: web::Data<Addr<PgConnection>>, request: web::Json<LoginRequest>) -> HttpResponse {
    let response = db.send(request.0).await.unwrap();
    if response.is_err() {
        return HttpResponse::Unauthorized().finish();
    }
    let user_id = response.unwrap();
    let role = db.send(UserRoleRequest::new(user_id)).await.unwrap();
    if role.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let role = role.unwrap();
    let token = generate_token(user_id, role);
    let body = serde_json::to_string(&LoginResponse {
        token,
        role,
    }).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}
