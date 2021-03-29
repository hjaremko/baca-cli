# baca-cli

### Running
Environment variable `BACA_SESSION` *must* be set to your BaCa `JSESSIONID` cookie.
```
BACA_SESSION=<session id> cargo run --release
```

### Setting log levels
Log levels are configured by setting environment variable `BACA_LOG`. For example:
```
BACA_LOG=trace cargo run
```
Available levels: `trace`, `debug`, `warn`, `error`, `info`.