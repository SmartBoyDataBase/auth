use std::io;

use actix::{Handler, Message, ResponseFuture};
use serde::{Deserialize, Serialize};

use crate::db::PgConnection;

#[derive(Deserialize, Serialize)]
pub struct UserRoleRequest(i64);

impl UserRoleRequest {
    pub fn new(user_id: i64) -> Self {
        Self(user_id)
    }
}

impl Message for UserRoleRequest {
    type Result = io::Result<i64>;
}

impl Handler<UserRoleRequest> for PgConnection {
    type Result = ResponseFuture<io::Result<i64>>;

    fn handle(&mut self, msg: UserRoleRequest, _: &mut Self::Context) -> Self::Result {
        let query_future = self.client
            .query_one(&self.role, &[&msg.0]);
        Box::pin(async move {
            let row = query_future.await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
            Ok(row.get(0))
        })
    }
}