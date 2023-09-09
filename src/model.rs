// models.rs

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::human;

#[derive(Queryable, Serialize)]
pub struct Human {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "human"]
pub struct NewHuman {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "human"]
pub struct UpdateHuman {
    first_name: Option<String>,
    last_name: Option<String>,
    age: Option<i32>,
}
