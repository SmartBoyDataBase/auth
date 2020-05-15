use std::io;

use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, NoTls, Statement};

pub struct PgConnection {
    pub client: Client,
    pub login: Statement,
    pub role: Statement,
}

impl Actor for PgConnection {
    type Context = Context<Self>;
}

impl PgConnection {
    pub async fn connect(db_url: &str) -> Result<Addr<PgConnection>, io::Error> {
        let (client, conn) = tokio_postgres::connect(db_url, NoTls)
            .await
            .expect("can not connect to postgresql");
        actix_rt::spawn(conn.map(|_| ()));
        let login = client.prepare("SELECT id FROM \"User\" \
                WHERE username=$1 \
                AND password=crypt($2, password);").await.unwrap();
        let role = client.prepare("SELECT role_id FROM user_role \
                WHERE user_id=$1;").await.unwrap();
        Ok(PgConnection::create(move |_| PgConnection {
            client,
            login,
            role,
        }))
    }
}

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