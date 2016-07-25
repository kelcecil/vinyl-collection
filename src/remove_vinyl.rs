extern crate pencil;
extern crate diesel;

use pencil::{Request, Response, PencilResult};

use diesel::prelude::*;

use establish_connection;

pub fn remove_vinyl_from_collection(req: &mut Request) -> PencilResult {
    use schema::vinyl::dsl::*;
    let conn = establish_connection();

    let vinyl_id = req.view_args
        .get("vinyl_id")
        .ok_or("No vinyl id provided.");

    // Handle the unwrap properly.
    let unwrapped_identifier = match vinyl_id {
        Ok(identifier) => identifier.parse::<i32>().unwrap(),
        Err(err) => return Ok(Response::from(err)),
    };

    let records_modified =
        diesel::delete(vinyl.filter(id.eq(unwrapped_identifier)))
                .execute(&conn)
                .expect("Error deleting vinyl.");

    Ok(Response::from(records_modified.to_string()))
}
