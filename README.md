# baca-cli [![Build](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml/badge.svg)](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml)

CLI client for the Jagiellonian University's BaCa online judge

<img src="https://i.imgur.com/qqkTrDa.gif" align="right" alt="UJ" title="Jagiellonian University"/>

![Preview](https://i.imgur.com/jl7j72k.png)

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
    refresh    Refreshes session, use in case of cookie expiration
```

### Workspace initialization: `init`

Initializes current directory as BaCa workspace, similar to `git init`. Currently passwords are stored in **plain text**
.

```
baca init --host <host> --login <login> --password <password>
```

```
-h, --host <host>           BaCa hostname, ex. mn2020
-p, --perm <permutation>    BaCa login
-s, --session <session>     BaCa password
```

Example, running on `Metody numeryczne 2019/2020`:

```
baca init --host mn2020 --login jaremko --password PaSsWorD
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

### Re-login: `refresh`

Refreshes session, use in case of cookie expiration.

```
baca refresh
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
