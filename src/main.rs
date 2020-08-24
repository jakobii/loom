
const DIGITAL_OCEAN_CONN_STR: &str = "";

use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fs;
use tokio_postgres;
use uuid::Uuid;
use futures;

mod person;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert = fs::read("ca-certificate.crt")?;
    let cert = Certificate::from_pem(&cert)?;
    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;
    let connector = MakeTlsConnector::new(connector);

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(DIGITAL_OCEAN_CONN_STR, connector).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    //let p = get_person(&client, id).await;
    //println!("{:?}", p);

    let tb_people = person::TableRef::new(&client).await;

    let id = Uuid::parse_str("16c5fb08-d2bb-11ea-a425-77874abfec0e").unwrap();
    let p = futures::executor::block_on(tb_people.select_one(&id)).unwrap();
    println!("{:?}", p);

    // update
    let mut p2 = p.clone();
    p2.name = "Frank".to_string();
    println!(
        "{:?}",
        futures::executor::block_on(tb_people.update_one(&p2))
    );
    println!(
        "{:?}",
        futures::executor::block_on(tb_people.select_one(&id)).unwrap()
    );

    // delete
    println!(
        "{:?}",
        futures::executor::block_on(tb_people.delete_one(&id))
    );
    println!(
        "{:?}",
        assert!(futures::executor::block_on(tb_people.select_one(&id)).is_none())
    );

    // insert
    println!(
        "{:?}",
        futures::executor::block_on(tb_people.insert_one(&p))
    );
    println!(
        "{:?}",
        futures::executor::block_on(tb_people.select_one(&id)).unwrap()
    );

    Ok(())
}
