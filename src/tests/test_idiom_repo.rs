#![allow(unused_imports)]
use std::process::{self, Command, Stdio};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres;

use crate::{
    get_db_pool_conn,
    idiom_resource::repository::{select_all_idioms, select_all_idioms_not_read},
    models::{IdiomDetailEntity, IdiomRequestEntity, IdiomRequestMutations},
    run_migrations,
};

#[tokio::test]
async fn test_select_idioms() {
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

    run_migrations(conn.clone()).await;

    let idioms_list = match select_all_idioms(conn.clone()).await {
        Ok(list) => list,
        Err(err) => {
            println!("db error");
            println!("{:?}", err);
            process::exit(1);
        }
    };

    assert_eq!(idioms_list.len(), 199);
}

#[tokio::test]
async fn test_insert_idiom_req_tbl() {
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

    run_migrations(conn.clone()).await;

    let idiom_req_entity = IdiomRequestEntity {
        req_user: "1".to_string(),
        is_read: false,
        idiom: IdiomDetailEntity {
            id: "r4UwUyMzB3AB9MoY".to_string(),
            idiom_eng: "test".to_string(),
            idiom_hin: "hindi".to_string(),
        },
    };

    match idiom_req_entity.insert(conn.clone()).await {
        Ok(_) => (),
        Err(err) => {
            println!("Error Insert into requesting table");
            println!("{:?}", err);
            process::exit(1);
        }
    }

    let idiom_req_entity_list = match select_all_idioms_not_read(conn.clone()).await {
        Ok(rows) => rows,
        Err(err) => {
            println!("Error Selecting from requesting table");
            println!("{:?}", err);
            process::exit(1);
        }
    };

    assert_eq!(idiom_req_entity_list.len(), 1);
    assert_eq!(idiom_req_entity_list[0].idiom.id, "r4UwUyMzB3AB9MoY");
}

#[tokio::test]
async fn test_update_idiom_req_tbl() {
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

    run_migrations(conn.clone()).await;

    let idiom_req_entity = IdiomRequestEntity {
        req_user: "1".to_string(),
        is_read: false,
        idiom: IdiomDetailEntity {
            id: "r4UwUyMzB3AB9MoY".to_string(),
            idiom_eng: "test".to_string(),
            idiom_hin: "hindi".to_string(),
        },
    };

    match idiom_req_entity.insert(conn.clone()).await {
        Ok(_) => (),
        Err(err) => {
            println!("Error Insert into requesting table");
            println!("{:?}", err);
            process::exit(1);
        }
    }

    match idiom_req_entity.update(conn.clone()).await {
        Ok(_) => (),
        Err(err) => {
            println!("Error Insert into requesting table");
            println!("{:?}", err);
            process::exit(1);
        }
    }

    let idiom_req_entity_list = match select_all_idioms_not_read(conn.clone()).await {
        Ok(rows) => rows,
        Err(err) => {
            println!("Error Selecting from requesting table");
            println!("{:?}", err);
            process::exit(1);
        }
    };

    assert_eq!(idiom_req_entity_list.len(), 0);
    assert_eq!(idiom_req_entity_list[0].idiom.id, "r4UwUyMzB3AB9MoY");
}
