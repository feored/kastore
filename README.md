# Anduran

Anduran is a desktop application for inspecting and working with fheroes2 save
data.

The repository is a Rust workspace with two main parts:

```text
.
|-- app/                # Tauri + SvelteKit desktop app
|   |-- package.json    # Frontend and Tauri commands
|   |-- src/            # Svelte UI
|   `-- src-tauri/      # Rust desktop shell
`-- kastore/            # Rust library for reading/writing fheroes2 saves
```

Decoding/encoding of the save file and validation should stay in Kastore.

## Usage

Install frontend dependencies from the app directory:

```sh
cd app
pnpm install
```

Start the desktop application:

```sh
pnpm tauri dev
```
