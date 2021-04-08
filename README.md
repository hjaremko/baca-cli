# baca-cli [![Build](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml/badge.svg)](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml)

<img src="https://i.imgur.com/qqkTrDa.gif" align="right" alt="UJ" title="Jagiellonian University"/>

CLI client for the Jagiellonian University's BaCa online judge

### Running

```
cargo run --release -- init --host <BaCa instance name> --perm <BaCa instance permutation> --session <JSESSIONID cookie>
```

#### Example, running on `Metody numeryczne 2019/2020`

```
cargo run --release -- init --host mn2020 --perm 5A4AE95C27260DF45F17F9BF027335F6 --session BC41D1615839AE5D7883EE62D49BCFE2
```

### Dependencies (Linux only)

```
sudo apt install pkg-config libssl-dev
```

### Setting log levels

Log levels are configured by `-v` flag.

- `no flag` - **warn**
- `-v` - **info**
- `-vv` - **debug**
- `-vvv or more` - **trace**
