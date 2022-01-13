[![Statix](https://circleci.com/gh/statix-server/statix.svg?style=svg)](https://github.com/statix-server/statix/)
         Made with ❤ by [@Octalbyte](https://github.com/Octalbyte/)
# statix
Static file server (Rust).

# Install and Run

```bash
cargo install statix
statix #On the folder you want to host

```

# Features and switches

- `--host <String>`  set host
- `--port <i32>` set port
- `--ssl <bool>` set if server should have ssl, then set file paths with `--crt <String>` (default cert.pem) and `--key <String>` (default key.pem)
- `--threads<i32>` set number of threads to be spawned

# WIP

- Automatically compile markdown files
