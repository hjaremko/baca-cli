# baca-cli [![Build](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml/badge.svg)](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml)

<img src="https://i.imgur.com/qqkTrDa.gif" align="right" alt="UJ" title="Jagiellonian University"/>

CLI client for the Jagiellonian University's BaCa online judge

## Usage

```
baca [FLAGS] [SUBCOMMAND]
```

```
FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

SUBCOMMANDS:
    details    Gets submit details
    help       Prints this message or the help of the given subcommand(s)
    init       Initializes current directory as BaCa workspace
```

### Workspace initialization: `init`

Initializes current directory as BaCa workspace, similar to `git init`.

```
baca init --host <host> --perm <permutation> --session <session>
```

```
-h, --host <host>           BaCa hostname, ex. mn2020
-p, --perm <permutation>    BaCa host permutation, found in 'X-GWT-Permutation' header of HTTP request
-s, --session <session>     BaCa session cookie, found in 'JSESSIONID' cookie of HTTP request
```

Example, running on `Metody numeryczne 2019/2020`:

```
baca init --host mn2020 --perm 5A4AE95C27260DF45F17F9BF027335F6 --session BC41D1615839AE5D7883EE62D49BCFE2
```

### Submit details: `details`

Prints details of given submit. Requires initialized workspace.

```
baca details <id>
```

Example:

```
> baca details 4334

● 100% - [G] Funkcje sklejane - submit 4334
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/4334
```

## Compilation

```
cargo build --release
```

### Dependencies (Linux only)

```
sudo apt install pkg-config libssl-dev
```

## Setting log levels

Log levels are configured by `-v` flag.

- `no flag` - **warn**
- `-v` - **info**
- `-vv` - **debug**
- `-vvv or more` - **trace**
