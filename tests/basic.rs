use entitystorage::{self, EntityDB};
mod common;
use common::*;

#[test]
fn insert_and_lookup_again(){
  let db = EntityDB::new();

  db.add(Person::new("Anders"));
  db.add(Person::new("Kim"));
  db.add(Point::new(123, 5));

  let person = db.lookup_id::<Person>(2);
  assert!(person.is_some());

  let mut person = person.unwrap();
  assert_eq!(person.data.name, "Kim");

  person.data.roles.push("admin".to_owned());
  db.save(person.clone());

  let person = db.lookup_id::<Person>(2);
  assert!(person.is_some());

  assert_eq!(person.unwrap().data.roles[0], "admin");
}

#[test]
fn serialize_json(){
  let db = EntityDB::new();
  db.add(Person::new("Anders"));
  let mut person = db.lookup_id::<Person>(1).unwrap();
  person.data.roles.push("admin".to_owned());
  person.data.points.add(&db.add(Point::new(555, 777)));
  let serialized = serde_json::to_string(&person.data).unwrap();
  assert_eq!(serialized, "{\"name\":\"Anders\",\"roles\":[\"admin\"],\"points\":{\"ids\":[2]}}");
}

#[test]
fn rels(){
  let db = EntityDB::new();
  db.add(Person::new("Anders"));
  let mut person = db.lookup_id::<Person>(1).unwrap();
  person.data.points.add(&db.add(Point::new(555, 777)));
  db.save(person);
  let person = db.lookup_id::<Person>(1).unwrap();
  assert_eq!(person.data.points.resolve_first::<Point>(&db).unwrap().data.x, 555)
}

#[test]
fn search(){
  let db = EntityDB::new();
  db.add(Person::new("Anders"));
  db.add(Person::new("Kim"));
  db.add(Person::new("Bo"));
  let persons = db.search::<Person, _>(|p| p.name == "Kim" || p.name == "Bo");
  let names: String = persons.iter().map(|p| p.data.name.clone()).into_iter().collect();
  assert!(names == "KimBo" || names == "BoKim");
}

#[test]
fn all(){
  let db = EntityDB::new();
  db.add(Person::new("Anders"));
  db.add(Person::new("Kim"));
  db.add(Person::new("Bo"));
  db.add(Point::new(123, 5));
  let persons = db.all::<Person>();
  assert_eq!(persons.len(), 3);
}

#[test]
fn user_roles(){

  use entitystorage::{Deserialize, Serialize, Rels};
  #[derive(Serialize, Deserialize, Clone)]
  struct Role{
    name: &'static str,
    permissions: Vec<&'static str>
  }
  #[derive(Serialize, Deserialize, Clone)]
  struct User{
    name: &'static str,
    roles: Rels
  }

  let db = EntityDB::new();
  db.add(User{name: "Anders", roles: Rels::new()});
  let mut user = db.lookup_id::<User>(1).unwrap();
  user.data.roles.add(&db.add(Role{name: "Admin", permissions: vec!["user.add", "user.delete"]}));
  db.save(user);
  let user = db.lookup_id::<User>(1).unwrap();
  assert_eq!(user.data.roles.resolve_first::<Role>(&db).unwrap().data.name, "Admin");
}

#[test]
fn chaining_and_rels(){
  let db = EntityDB::new();
  let mut person = db.add(Person::new("Anders"));
  person
    .data
    .points
    .add(&db.add(Point::new(1, 2)));

  db.save(person);  
  let person = db.lookup_id::<Person>(1).unwrap();
  assert_eq!(person.data.points.resolve_first::<Point>(&db).unwrap().data.y, 2);
  assert_eq!(person.data.points.resolve::<Point>(&db).len(), 1);
  
}