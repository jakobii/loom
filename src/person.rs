use serde::Deserialize;
use tokio_postgres;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

const SQL_INSERT_ONE: &str = r#"
INSERT INTO people (
    id,
    fn
) VALUES (
    $1::UUID,
    $2::TEXT
);
"#;

const SQL_SELECT_ONE: &str = r#"
SELECT 
    id, 
    fn
FROM people
WHERE
    id = $1::UUID;
"#;

const SQL_UPDATE_ONE: &str = r#"
UPDATE people 
SET
    fn = $2::TEXT
WHERE
    id = $1::UUID;
"#;

const SQL_DELETE_ONE: &str = r#"
DELETE
FROM people
WHERE
    id = $1::UUID;
"#;

pub struct TableRef <'a> {
    client: &'a tokio_postgres::Client,
    smt_insert_one: tokio_postgres::Statement,
    smt_select_one: tokio_postgres::Statement,
    smt_update_one: tokio_postgres::Statement,
    smt_delete_one: tokio_postgres::Statement,
}

impl<'a> TableRef<'a> {
    pub async fn new(client: &'a tokio_postgres::Client) -> TableRef <'a> {
        TableRef {
            client: client,
            smt_insert_one: client.prepare(SQL_INSERT_ONE).await.unwrap(),
            smt_select_one: client.prepare(SQL_SELECT_ONE).await.unwrap(),
            smt_update_one: client.prepare(SQL_UPDATE_ONE).await.unwrap(),
            smt_delete_one: client.prepare(SQL_DELETE_ONE).await.unwrap(),
        }
    }
    pub async fn insert_one(&self, p: &Person) -> bool {
        let result = self
            .client
            .execute(&self.smt_insert_one, &[&p.id, &p.name])
            .await;

        match result {
            Ok(_) => {
                //println!("inserted: {:?}", n);
                true
            }
            Err(_) => {
                //println!("{:?}", e);
                false
            }
        }
    }
    pub async fn select_one(&self, id: &Uuid) -> Option<Person> {
        let result = self.client.query_one(&self.smt_select_one, &[id]).await;
        match result {
            Ok(row) => Some(Person {
                id: row.get(0),
                name: row.get(1),
            }),
            Err(_) => {
                //println!("{:?}", e);
                None
            }
        }
    }
    pub async fn update_one(&self, p: &Person) -> bool {
        let result = self
            .client
            .execute(&self.smt_update_one, &[&p.id, &p.name])
            .await;

        match result {
            Ok(_) => {
                //println!("update: {:?}", n);
                true
            }
            Err(_) => {
                //println!("{:?}", e);
                false
            }
        }
    }
    pub async fn delete_one(&self, id: &Uuid) -> bool {
        let result = self.client.execute(&self.smt_delete_one, &[id]).await;

        match result {
            Ok(_) => {
                //println!("deleted {:?}", n);
                true
            }
            Err(_) => {
                //println!("{:?}", e);
                false
            }
        }
    }
}
