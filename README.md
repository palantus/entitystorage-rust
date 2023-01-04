# entitystorage-rust
Entitystorage in Rust

### Data types

Creating data types (structs) can be done using the following code. Note that they all must derive Serialize, Deserialize (both from serde) and Clone.

```rust
use entitystorage::{Deserialize, Serialize, EntityDB};

#[derive(Serialize, Deserialize, Clone)]
struct User{
  name: String
}
```

These can then be saved in the database using `save`:

```rust
let mut db = EntityDB::new();
db.add(User{name: "Anders".to_owned()});
```