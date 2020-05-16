use std::io;

use actix::{Actor, Addr, Context};
use futures::FutureExt;
use tokio_postgres::{Client, NoTls, Statement};

pub mod login;
pub mod user_role;
pub mod create;

pub struct PgConnection {
    pub client: Client,
    pub login: Statement,
    pub role: Statement,
    pub create: Statement,
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
        let create = client.prepare("\
        INSERT INTO \"User\"(username, password)\
        VALUES ($1, crypt($2, gen_salt('bf')))\
        RETURNING id;\
        ").await.unwrap();
        Ok(PgConnection::create(move |_| PgConnection {
            client,
            login,
            role,
            create,
        }))
    }
}
