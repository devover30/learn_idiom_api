#![allow(unused_imports)]
use std::process;

use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres;

use crate::{get_db_pool_conn, run_migrations};

#[tokio::test]
async fn test_container() {
    let node = postgres::Postgres::default().start().await;

    // Get the PostgreSQL port
    let pg_port = node.get_host_port_ipv4(5432).await;

    // prepare connection string
    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        pg_port
    );
    // container is up, you can use it
    let conn = match get_db_pool_conn(connection_string.to_owned()).await {
        Ok(conn) => conn,
        Err(err) => {
            println!("Error Connecting to Database");
            println!("{:?}", err);
            process::exit(1);
        }
    };

    run_migrations(conn).await;

    assert_eq!(1, 1);
}
