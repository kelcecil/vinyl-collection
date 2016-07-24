extern crate pencil;
extern crate diesel;

use pencil::{Request, Response, PencilResult, jsonify};
use models::{Vinyl, NewVinyl};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use establish_connection;

pub fn add_new_vinyl(req: &mut Request) -> PencilResult {
    let conn = establish_connection();

    let json = req.get_json();
    match *json {
        Some(ref x) => {
            let title = match x.search("title").ok_or("No album title provided.") {
                Ok(val) => val.as_string().unwrap(),
                Err(err) => return Ok(Response::from(err)),
            };
            let artist = match x.search("artist").ok_or("No artist name provided.") {
                Ok(val) => val.as_string().unwrap(),
                Err(err) => return Ok(Response::from(err)),
            };

            let added_vinyl = create_vinyl(&conn, title, artist);
            jsonify(&added_vinyl)
        },
        None => Ok(Response::from("No JSON")),
    }
}

pub fn create_vinyl<'a>(conn: &PgConnection, title: &'a str, artist: &'a str) -> Vinyl {
    use schema::vinyl;

    let new_vinyl = NewVinyl {
        title: title,
        artist: artist,
    };

    diesel::insert(&new_vinyl).into(vinyl::table)
        .get_result(conn)
        .expect("Error saving new post")
}
