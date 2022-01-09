[![Statix](https://circleci.com/gh/statix-server/statix.svg?style=svg)](https://github.com/statix-server/statix/)
Made with love by [@Octalbyte](https://github.com/Octalbyte/)
# statix
Static file server (Rust).

# Install

```bash
cargo install statix
statix

```

# Features and switches

- `--host <String>`  set host
- `--port <i32>` set port
- `--ssl <bool>` set if server should have ssl, then set file paths with `--crt <String>` and `--key <String>`
- `--threads<i32>` set number of threads to be spawned

# WIP

- --spa option
- Automatically compile markdown files