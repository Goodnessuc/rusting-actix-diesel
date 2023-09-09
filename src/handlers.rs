use crate::database::establish_connection;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::model::{Human, NewHuman, UpdateHuman};

// READ ALL
pub async fn get_humans() -> Result<HttpResponse> {
    use crate::schema::human::dsl::*;

    let mut connection = establish_connection();
    let humans = human
        .load::<Human>(&mut connection)
        .expect("Error loading humans");

    Ok(HttpResponse::Ok().json(humans))
}


// UPDATE
pub async fn update_human(
    id: web::Path<i32>,
    human_update: web::Json<UpdateHuman>,
) -> Result<HttpResponse> {
    use crate::schema::human::dsl::*;

    let mut connection = establish_connection();

    // Use the `update` method of the Diesel ORM to update the student's record
    let updated_human = diesel::update(human.find(id))
        .set(&human_update.into_inner())
        .execute(&mut connection)
        .expect("Failed to update student");

    Ok(HttpResponse::Ok().json(updated_human))
}


pub async fn create_human(new_human: web::Json<NewHuman>) -> Result<HttpResponse> {
    use crate::schema::human::dsl::*;

    let mut connection = establish_connection();

    diesel::insert_into(human)
        .values(&new_human.into_inner())
        .execute(&mut connection)
        .expect("Error inserting new human");

    Ok(HttpResponse::Ok().json("data inserted into the database"))
}

// DELETE
pub async fn delete_human(id: web::Path<i32>) -> Result<HttpResponse> {
    use crate::schema::human::dsl::*;
    let mut connection = establish_connection();

    diesel::delete(human.find(id))
        .execute(&mut connection)
        .expect(&format!("Unable to find student {:?}", id));

    Ok(HttpResponse::Ok().json("Deleted successfully"))
}