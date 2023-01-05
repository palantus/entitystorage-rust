use entitystorage::{Deserialize, Serialize, Rels};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Point{
    pub x: i32,
    pub y: i32,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub roles: Vec<String>,
    pub points: Rels
}

impl Point{
  pub fn new(x: i32, y: i32) -> Point{
    Point{x, y}
  }
}
impl Person{
  pub fn new(name: &str) -> Person{
    Person{name: name.to_owned(), roles: vec![], points: Rels::new()}
  }
}