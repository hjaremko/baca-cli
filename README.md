# baca-cli [![Build](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml/badge.svg)](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml) [![codecov](https://codecov.io/gh/hjaremko/baca-cli/branch/master/graph/badge.svg?token=CP9EWDCOMV)](https://codecov.io/gh/hjaremko/baca-cli)

CLI client for the Jagiellonian University's BaCa online judge

![Preview](https://i.imgur.com/xOAHuXk.png)

## Installation

The latest release can be downloaded **[here](https://github.com/hjaremko/baca-cli/releases)**.

- **Windows** users can use convenient installer or download raw binary.
- **Linux** and **macOS** users should rename binary to `baca` and copy it to `~/.local/bin` or whatever your `PATH` is.
- **Cargo** users can install with command `cargo install --git https://github.com/hjaremko/baca-cli.git`

### Arch Linux

You can download the latest release from [AUR](https://aur.archlinux.org/packages/baca-cli) and install it using your
favourite AUR helper or directly from source:

```
sudo pacman -S base-devel git
git clone https://aur.archlinux.org/baca-cli.git
cd baca-cli
makepkg -sic
```

### Dependencies (Linux)

```
sudo apt install pkg-config libssl-dev
```

## Troubleshooting

- **[Deprecated TLSv1 protocol](#troubleshooting)**

## Usage

```
baca [FLAGS] [SUBCOMMAND]
```

```
FLAGS:
    -U, --force-update    Force update check
    -h, --help            Prints help information
    -u, --no-update       Disable update check
    -V, --version         Prints version information
    -v, --verbose         Sets the level of verbosity

SUBCOMMANDS:
    clear      Removes the whole `.baca` directory
    config     Opens editor to edit BaCa configuration
    details    Gets submit details
    help       Prints this message or the help of the given subcommand(s)
    init       Initializes current directory as BaCa workspace
    last       Prints details of the last submit
    log        Prints last (default 3) submits
    refresh    Refreshes session, use in case of cookie expiration
    submit     Submits file
    tasks      Prints available tasks
```

### Workspace initialization: `init`

Initializes current directory as BaCa workspace, similar to `git init`. Currently, passwords are stored in **plain
text.**
User will be asked for credentials, if not provided.

```
baca init
```

```
-h, --host <host>            BaCa hostname, ex. mn2020
-l, --login <login>          BaCa login
-p, --password <password>    BaCa password
```

Example, running on `Metody numeryczne 2019/2020` with no login prompt:

```
baca init --host mn2020 --login jaremko --password PaSsWorD
```

### Re-login: `refresh`

Refreshes session, use in case of cookie expiration.

```
baca refresh
```

### Submit: `submit`

Submits given file to specified task. Will prompt the user for task, if not provided.

- Optional parameter `--task <id>` explicitly sets problem to submit to. Use `baca tasks` to see what ids are available.
- Optional parameter `--zip` will zip given file before submitting. The archive is saved as **`source.zip`**.
- Optional parameter `--rename` will rename file before submitting and zipping.
- Optional parameter `--no-main` will remove main function from C/C++ files before submitting and zipping.
- Optional parameter `--language <language>` explicitly sets input file language.
- `submit config` opens editor to edit submit config.
- `submit clear` clears saved submit config.

```
USAGE:
    baca submit [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
        --no-main    Removes main function before submitting. Takes effect only on C/C++ files.
        --no-save    Does not ask for save
    -s, --save       Saves task config, if provided, future 'submit' calls won't require providing task config
    -V, --version    Prints version information
    -z, --zip        Zips files to 'source.zip' before submitting, overrides saved config

OPTIONS:
    -f, --file <file>            File to submit, overrided saved path
    -l, --language <language>    Task language. Please type exacly as it is displayed on Baca.
    -r, --rename <rename>        Submit input file under different name
    -t, --task-id <task_id>      Task id, type 'baca tasks' to see what ids are available, overrides saved task id

SUBCOMMANDS:
    clear     Clears saved submit config
    config    Opens editor to edit submit config
    help      Prints this message or the help of the given subcommand(s)

```

Example:

```
> baca submit -f hello.cpp
✔ Choose task: · [E] Metoda SOR
Submitting hello.cpp to task [E] Metoda SOR (C++ with file support).
```

#### Saving task info

If you don't want to type task info (id and filename) every time you submit, you can use `--save` flag to save it. Keep
in mind, that config provided through parameters will override saved data. To completely remove saved data
use `baca submit clear`. To disable automatic prompt for save, use `--no-save`.

Example:

```
> baca submit -f hello.cpp -t 5 --save
Submitting hello.cpp to task [E] Metoda SOR (C++ with file support).
> baca submit
Submitting hello.cpp to task [E] Metoda SOR (C++ with file support).
```

### Recent submits: `log`

Prints statuses of a couple of recent submits (default 3). Parameter `-t <task_id>` lets you print logs for specific
task. Task ID can be found by `baca tasks`.

```
baca log [optional: number] [optional: -t <task_id>]
```

Example:

```
> baca log

● [G] Funkcje sklejane - C++ - 2020-05-17 18:53:09 - submit 4334
├─── 100% - 4 pts - Ok
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/4334

● [G] Funkcje sklejane - C++ - 2020-05-17 16:57:22 - submit 4328
├─── 100% - 4 pts - Ok
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/4328

● [G] Funkcje sklejane - C++ - 2020-05-17 16:53:41 - submit 4326
├─── 0% - 0 pts - WrongAnswer
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/4326
```

### Last submit details: `last`

Prints details of last submit. Requires workspace to be initialized. Parameter `-t <task_id>` lets you print logs for
specific task. Task ID can be found by `baca tasks`.

```
baca last [optional: -t <task_id>]
```

Example:

```
> baca last

● [G] Funkcje sklejane - C++ - 2020-05-17 18:53:09 - submit 4334
├─── 100% - 4/4 pts - Ok
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/4334
 ✔️ ── test0/0 - Ok
 ✔️ ── test1/0 - Ok
 ✔️ ── test2/0 - Ok
 ✔️ ── test3/0 - Ok
```

### Any submit details: `details`

Prints details of given submit. Requires workspace to be initialized.

```
baca details <id>
```

Example:

```
> baca details 2904

● [D] Skalowany Gauss - C++ - 2020-04-22 19:20:07 - submit 2904
├─── 89% - 3.58/4 pts - TimeExceeded
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/2904
 ✔️ ── testy_jawne/test1 - Ok
 ✔️ ── testy_jawne/test2 - Ok
 ✔️ ── testy_jawne/test3 - Ok
 ✔️ ── testy_jawne/test4 - Ok
 ✔️ ── testy_jawne/test5 - Ok
 ✔️ ── testy_jawne/test6 - Ok
 ✔️ ── testy_jawne/test8 - Ok
 ✔️ ── testy/test0 - Ok
 ✔️ ── testy/test1 - Ok
 ❌  ── testy/test10 - TimeExceeded
 ❌  ── testy/test11 - TimeExceeded
 ✔️ ── testy/test2 - Ok
 ✔️ ── testy/test3 - Ok
 ✔️ ── testy/test4 - Ok
 ✔️ ── testy/test5 - Ok
 ✔️ ── testy/test6 - Ok
 ✔️ ── testy/test7 - Ok
 ✔️ ── testy/test8 - Ok
 ✔️ ── testy/test9 - Ok
```

### All tasks: `tasks`

Prints all tasks.

```
baca tasks
```

Example:

```
> baca tasks

● 1 - [A] Zera funkcji - 69 OK
● 2 - [B] Metoda Newtona - 58 OK
● 3 - [C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane - 62 OK
● 4 - [D] Skalowany Gauss - 52 OK
● 5 - [E] Metoda SOR - 64 OK
● 6 - [F] Interpolacja - 63 OK
● 7 - [G] Funkcje sklejane - 59 OK
● 8 - A2 - 1 OK
● 9 - B2 - 2 OK
● 10 - C2 - 1 OK
● 11 - D2 - 2 OK
● 12 - E2 - 1 OK
● 13 - F2 - 3 OK
● 14 - G2 - 2 OK
```

## Environment variables

### Settings for update check

```
GITHUB_USER=hjaremko
GITHUB_REPO=baca-cli
AUTH_TOKEN=<github token> # auth GitHub API requests (increases API call limit)
```

## Compilation

```
cargo build --release
```

## Running tests

Some tests require credentials to actual BaCa server, which can be set using environment variables. These tests are
disabled by default, you can try running them with command `cargo test -- --ignored`.

```
TEST_BACA_LOGIN=<login>
TEST_BACA_PASSWORD=<password>
TEST_BACA_HOST=<host>
```

## Setting log levels

Log levels are configured by `-v` flag.

- `no flag` - no additional logs
- `-v` - **info**
- `-vv` - **debug**
- `-vvv or more` - **trace**

## Troubleshooting

- `Unfortunately, Baca still uses deprecated TLSv1 protocol which is not supported on your system. Sorry!`  
  Since Baca uses deprecated TLSv1 protocol, you have to enable TLSv1 to use it. You can do it multiple ways, some of
  which are described below.
    - Every method presented below assumes that a OpenSSL config file exists at `~/.openssl.cnf`. Inside it, you can
      specify the security level, which changes the TLS version. You can find more information by
      clicking [here](https://discourse.ubuntu.com/t/default-to-tls-v1-2-in-all-tls-libraries-in-20-04-lts/12464/8).

      **Example config:**

        ```
        openssl_conf = openssl_init
        
        [openssl_init]
        ssl_conf = ssl_sect
        
        [ssl_sect]
        system_default = system_default_sect
        
        [system_default_sect]
        CipherString = DEFAULT@SECLEVEL=1
        ```

    - Supplying the environmental variable only for the command that follows: `OPENSSL_CONF=~/.openssl.cnf baca`.
    - Creating an alias in your shell's RC file. This enables this specific OpenSSL config for this specific terminal
      instance, until it's closed.  
      **Example:**
        1. In `.bashrc` append `alias bacaSSL="export OPENSSL_CONF=~/.openssl_baca.cnf"`.
        2. Restart the terminal or `source /path/to/.bashrc`.
        3. Use baca-cli as you would normally.
    - ***Not recommended!*** It is possible to overwrite the global OpenSSL config to allow for TLSv1 connections
      everywhere, but it is **really** insecure to do so. Thus, we will not show how to do it.
