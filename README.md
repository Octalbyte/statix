[![Statix](https://circleci.com/gh/Octalbyte/statix.svg?style=svg)](https://github.com/Octalbyte/statix/)
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
- `--cors <String>` set the value of header `Access-Control-Allow-Origin`
- `--blocktor` Block requests made through TOR
- `--username <String>` Username for authentication (If None, there will be no authentication required)
- `--pwd <String>` Password for authentication. 
- `--help` display more help
