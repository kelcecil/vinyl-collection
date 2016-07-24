use super::schema::*;

#[derive(Queryable)]
#[derive(RustcEncodable)]
pub struct Vinyl {
    pub id: i32,
    pub title: String,
    pub artist: String,
}

#[insertable_into(vinyl)]
pub struct NewVinyl<'a> {
    pub title: &'a str,
    pub artist: &'a str,
}
