# json-logger

Logs a specified key of the JSON input from stdin to a directory. The log file is based on the
current directory path and the lines are prefixed with a time-stamp.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam json-logger
```

## Usage

```
echo '{"msg": "A new log entry"}' | cargo run -- --key msg --log-dir /tmp/json-logs
```

This logs to a file named `/tmp/json-logs/-path-to-current-dir`.
