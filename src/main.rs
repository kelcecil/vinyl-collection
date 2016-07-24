extern crate pencil;
extern crate vinyl_collection;

use pencil::{Pencil};
use self::vinyl_collection::new_vinyl::add_new_vinyl;

fn main() {
    let mut app = Pencil::new("/web/hello");
    app.post("/vinyl/new", "new_vinyl", add_new_vinyl);
    println!("Running app");
    app.run("127.0.0.1:5000");
}
