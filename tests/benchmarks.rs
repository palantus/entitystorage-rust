use entitystorage::{self, EntityDB};
mod common;
use common::*;
use std::time::{Instant};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
#[ignore]
fn benchmark_lookup_large(){
  let mut db = EntityDB::new();
  db.add(Person::new("Kim"));

  let start = Instant::now();
  for _i in 0..10000{
    db.add(Person::new(SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_nanos()
                                .to_string().as_str()));      
  }
  for _i in 0..1000000{
    db.add(Point::new(1, 2));      
  }
  db.add(Person::new("Bettina"));

  println!("Time adding data: {:?}", start.elapsed());
  println!("db person count:: {:?}", db.all::<Person>().len());
  println!("db point count:: {:?}", db.all::<Point>().len());

  let start = Instant::now();
  db.find::<Person, _>(|p| p.name == "Kim");
  println!("1st lookup:: {:?}", start.elapsed());
  let start = Instant::now();
  db.find::<Person, _>(|p| p.name == "Bettina");
  println!("2nd lookup:: {:?}", start.elapsed());
}