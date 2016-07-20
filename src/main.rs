extern crate pencil;

use pencil::{Pencil, Request, Response, PencilResult};

fn new_vinyl(req: &mut Request) -> PencilResult {
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

fn main() {
    let mut app = Pencil::new("/web/hello");
    app.post("/vinyl/new", "new_vinyl", new_vinyl);
    println!("Running app");
    app.run("127.0.0.1:5000");
}
