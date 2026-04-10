## Kastore

A Rust library for reading and writing [fheroes2](https://github.com/ihhub/fheroes2) save files.

Still early and incomplete.

### Status

Currently supported:

- save versions `10032` and `10033`
- outer save container decoding/encoding
- header / map-info decoding/encoding
- compressed body preservation
- partial world decoding, including tile data

### Usage

```rust
let bytes = std::fs::read("save.sav")?;
let save = kastore::load(&bytes)?;

println!("{save}");

let encoded = kastore::save(&save)?;
std::fs::write("save.sav", encoded)?;
```