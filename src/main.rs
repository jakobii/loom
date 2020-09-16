#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use serde_json;
use rocket_contrib;

struct DynamicForm {
    value: serde_json::Value,
}

impl<'f> rocket::request::FromForm<'f> for DynamicForm {
    // In practice, we'd use a more descriptive error type.
    type Error = ();

    fn from_form(items: &mut rocket::request::FormItems<'f>, strict: bool) -> Result<Self, ()> {
        let mut m = serde_json::Map::new();
        for item in items {
            let key = item.key.as_str().to_string();
            let val: serde_json::Value;
            match item.value.url_decode() {
                Ok(x) => {
                    val = serde_json::Value::String(x)
                },
                Err(_) => {val = serde_json::Value::Null}, 
            }
            m.insert(key, val);
        }
        if m.keys().len() > 0 {
            return Ok(DynamicForm{value:serde_json::Value::Object(m)});
        } 
        
        // return error if strict is true and no fields where provided
        if strict {
            return Err(());
        }

        return Ok(DynamicForm{value:serde_json::Value::Null});
    }
}

#[get("/<path..>")]
fn get_some(
    db_state: rocket::State<Arc<Mutex<HashMap<String,serde_json::Value>>>>,
    path: rocket::http::uri::Segments,
) -> rocket_contrib::json::Json<serde_json::Value> {
    let key = String::from(path.0);
    let db_arc = db_state.inner();
    let db_clone = Arc::clone(db_arc);
    let db = db_clone.lock().unwrap();
    let result = db.get(&key);
    match result {
        Some(m) => {
            return rocket_contrib::json::Json(m.clone());
        }
        None => {
            return rocket_contrib::json::Json(serde_json::Value::Null);
        }
    }
}

#[put("/<path..>?<props..>")]
fn put_some(
    db_state: rocket::State<Arc<Mutex<HashMap<String,serde_json::Value>>>>,
    path: rocket::http::uri::Segments,
    props: rocket::request::LenientForm<DynamicForm>,
) {
    let key = String::from(path.0);
    let db_arc = db_state.inner();
    let db_clone = Arc::clone(db_arc);
    let mut db_lock = db_clone.lock().unwrap();
    db_lock.insert(key, props.value.clone());
}

#[delete("/<path..>")]
fn delete_some(
    db_state: rocket::State<Arc<Mutex<HashMap<String,serde_json::Value>>>>,
    path: rocket::http::uri::Segments,
) {
    let key = String::from(path.0);
    let db_arc = db_state.inner();
    let db_clone = Arc::clone(db_arc);
    let mut db_lock = db_clone.lock().unwrap();
    db_lock.remove(&key);
}

fn main() {
    let mut db: Arc<Mutex<HashMap<String,serde_json::Value>>> = Arc::new(Mutex::new(HashMap::new()));
    rocket::ignite()
        .manage(db)
        .mount("/", routes![get_some,put_some,delete_some])
        .launch();
}