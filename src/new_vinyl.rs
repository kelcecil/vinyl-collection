extern crate pencil;

use pencil::{Request, Response, PencilResult};

pub fn add_new_vinyl(req: &mut Request) -> PencilResult {
    let json = req.get_json();
    match *json {
        Some(ref x) =>
            match x.search("name") {
                Some(name) => {
                    let unwrapped_name = name.as_string().unwrap();
                    Ok(Response::from(unwrapped_name))
                },
                None => Ok(Response::from("No name")),
            },
        None => Ok(Response::from("No JSON")),
    }
}
