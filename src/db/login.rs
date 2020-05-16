use std::io;

use actix::{Handler, Message, ResponseFuture};
use serde::{Deserialize, Serialize};

use crate::db::PgConnection;

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

impl Message for LoginRequest {
    type Result = io::Result<i64>;
}

impl Handler<LoginRequest> for PgConnection {
    type Result = ResponseFuture<io::Result<i64>>;

    fn handle(&mut self, msg: LoginRequest, _: &mut Self::Context) -> Self::Result {
        let query_future = self.client
            .query_one(&self.login, &[&msg.username, &msg.password]);
        Box::pin(async move {
            let row = query_future.await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
            Ok(row.get(0))
        })
    }
}
