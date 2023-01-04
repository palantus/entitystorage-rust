# entitystorage-rust
Entitystorage in Rust

### Data types

Creating data types (structs) can be done using the following code. Note that they all must derive Serialize, Deserialize (both from serde) and Clone.

```rust
#[derive(Serialize, Deserialize, Clone)]
struct User{
  name: String
}
```