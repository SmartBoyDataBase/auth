use actix::Addr;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::db::create::SignInRequest;
use crate::db::login::LoginRequest;
use crate::db::PgConnection;
use crate::db::user_role::UserRoleRequest;
use crate::service::generate_token;

pub async fn ping() -> impl Responder {
    "pong"
}

#[derive(Deserialize, Serialize)]
pub struct SignInResponse {
    id: i64,
}

pub async fn sign_in(db: web::Data<Addr<PgConnection>>, request: web::Json<SignInRequest>) -> HttpResponse {
    let response = db.send(request.0).await.unwrap();
    if let Ok(user_id) = response {
        let body = serde_json::to_string(&SignInResponse {
            id: user_id
        }).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    } else {
        HttpResponse::InternalServerError().finish()
    }
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
