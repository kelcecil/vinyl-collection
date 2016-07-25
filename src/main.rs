extern crate pencil;
extern crate vinyl_collection;

use pencil::{Pencil};
use self::vinyl_collection::new_vinyl::add_new_vinyl;
use self::vinyl_collection::remove_vinyl::remove_vinyl_from_collection;

fn main() {
    let mut app = Pencil::new("/web/hello");
    app.post("/vinyl/new", "new_vinyl", add_new_vinyl);
    app.delete("/vinyl/<vinyl_id:int>", "remove_vinyl", remove_vinyl_from_collection);
    println!("Running app");
    app.run("127.0.0.1:5000");
}
