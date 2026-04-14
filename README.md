## Kastore

Rust library for reading and writing [fheroes2](https://github.com/ihhub/fheroes2) save files.

Decodes into a typed model and re-encodes from it.

### Features

-   Supports versions `10032` and `10033`
-	All sections of the save format, including:
	-   header / map info
	-   container (compression + wrapper)
	-   settings
	-   game-over result
	-   world (heroes, castles, kingdoms, tiles, objects)
-	Basic validation (e.g heroes/castle data in kingdoms)
-	Strict/Permissive modes 


### Parsing

Strict by default:

``` rust
let save = kastore::load(&bytes)?;
```

Permissive mode (diagnostics):

``` rust
use kastore::{load_with_options, LoadOptions, ParseMode};

let report = load_with_options(&bytes, &LoadOptions::permissive())?;
let save = report.value;
```

Permissive mode reports issues but still fails on invalid structure.

### Usage

``` rust
let bytes = std::fs::read("save.sav")?;
let save = kastore::load(&bytes)?;

let encoded = kastore::save(&save)?;
std::fs::write("save.sav", encoded)?;
```

### Notes

-   encoding is model-driven, not byte-preserving
-   API not yet stable
