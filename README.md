# entitystorage-rust
Entitystorage in Rust

## Storing data

Creating data types (structs) can be done using the following code. Note that they all must derive Serialize, Deserialize (both from serde) and Clone.

```rust
use entitystorage::{Deserialize, Serialize, EntityDB};

#[derive(Serialize, Deserialize, Clone)]
struct User{
  name: String
}
```

These can then be saved in the database using `add`:

```rust
let mut db = EntityDB::new();
let mut user = db.add(User{name: "Anders".to_owned()});
```

After modifing an entity, you can save it using `db.save`:

```rust
user.data.name = "Linus".to_owned();
db.save(user);
```

## Finding entities

Lookup by id:
```rust
db.lookup_id::<User>(1);
```

Find all by property value:
```rust
let users = db.search::<User, _>(|p| p.name == "Anders");
```

Find first by property value:
```rust
let user = db.find::<User, _>(|p| p.name == "Anders");
```

Find all by a specific type:
```rust
let users = db.all::<User>();
```

## Relations

Introduce the `Role` struct and add `roles` to users:
```rust
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
```

Adding roles to a user:
```rust
let mut db = EntityDB::new();
db.add(User{name: "Anders", roles: Rels::new()});
let mut user = db.lookup_id::<User>(1).unwrap();
user.data.roles.add(&db.add(Role{name: "Admin", permissions: vec!["user.add", "user.delete"]}));
db.save(user);
```

Getting roles again can be done using `resolve` or `resolve_first`:

```rust
let all_roles = user.data.roles.resolve::<Role>(&db)
let first_role = user.data.roles.resolve_first::<Role>(&db).unwrap()
```
