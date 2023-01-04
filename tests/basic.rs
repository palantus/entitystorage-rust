use entitystorage::{self, EntityDB};
mod common;
use common::*;

#[test]
fn insert_and_lookup_again(){
  let mut db = EntityDB::new();

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
  let mut db = EntityDB::new();
  db.add(Person::new("Anders"));
  let mut person = db.lookup_id::<Person>(1).unwrap();
  person.data.roles.push("admin".to_owned());
  person.data.points.add(&db.add(Point::new(555, 777)));
  let serialized = serde_json::to_string(&person.data).unwrap();
  assert_eq!(serialized, "{\"name\":\"Anders\",\"roles\":[\"admin\"],\"points\":{\"ids\":[2]}}");
}

#[test]
fn rels(){
  let mut db = EntityDB::new();
  db.add(Person::new("Anders"));
  let mut person = db.lookup_id::<Person>(1).unwrap();
  person.data.points.add(&db.add(Point::new(555, 777)));
  db.save(person);
  let person = db.lookup_id::<Person>(1).unwrap();
  assert_eq!(person.data.points.resolve_first::<Point>(&db).unwrap().data.x, 555)
}

#[test]
fn search(){
  let mut db = EntityDB::new();
  db.add(Person::new("Anders"));
  db.add(Person::new("Kim"));
  db.add(Person::new("Bo"));
  let persons = db.search::<Person, _>(|p| p.name == "Kim" || p.name == "Bo");
  let names: String = persons.iter().map(|p| p.data.name.clone()).into_iter().collect();
  assert_eq!(names, "KimBo");
}