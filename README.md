# baca-cli [![Build](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml/badge.svg)](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml) [![codecov](https://codecov.io/gh/hjaremko/baca-cli/branch/master/graph/badge.svg?token=CP9EWDCOMV)](https://codecov.io/gh/hjaremko/baca-cli)

CLI client for the Jagiellonian University's BaCa online judge

![Preview](https://i.imgur.com/xOAHuXk.png)

## Installation

The latest release can be downloaded **[here](https://github.com/hjaremko/baca-cli/releases)**.

- **Windows** users can use convenient installer or download raw binary.
- **Linux** and **macOS** users should rename binary to `baca` and copy it to `~/.local/bin` or whatever your `PATH` is.
- **Cargo** users can clone the repo, then install with command  `cargo install --path .`

### Arch Linux
You can download the latest release from [AUR](https://aur.archlinux.org/packages/baca-cli) and install it using your favourite AUR helper or directly from source:
```
sudo pacman -S base-devel git
git clone https://aur.archlinux.org/baca-cli.git
cd baca-cli
makepkg -sic
```

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
    details    Gets submit details
    help       Prints this message or the help of the given subcommand(s)
    init       Initializes current directory as BaCa workspace
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

Submit file to task given by its id. Use `baca tasks` to see what ids are available.  
Passing optional parameter `--zip` will zip given file before submitting. The archive is saved as **`source.zip`**.  
**Currently a correct language string needs to be provided as well.**

```
baca submit -t <task_id> -f <filename> [optional --zip]
```

Example:

```
> baca submit -f hello.cpp -t 5 -l "C++ z obsluga plikow"

Submitting hello.cpp to task [E] Metoda SOR (C++ with file support).
```

#### Saving task info

If you don't want to type task info (id and filename) every time you submit, you can use `--default` flag to save it.
Keep in mind, that config provided though parameters will override saved data. To completely remove saved data
use `baca submit clear`.

Example:

```
> baca submit -f hello.cpp -t 5 --default
Submitting hello.cpp to task [E] Metoda SOR.
> baca submit
Submitting hello.cpp to task [E] Metoda SOR.
```

### Recent submits: `log`

Prints statuses of a couple of recent submits (default 3).

```
baca log [optional: number]
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

### Submit details: `details`

Prints details of given submit. Requires workspace to be initialized.

```
baca details <id>
```

Example:

```
> baca details 4334

● [G] Funkcje sklejane - C++ - 2020-05-17 18:53:09 - submit 4334
├─── 100% - 4 pts - Ok
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/4334
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

### Dependencies (Linux only)

```
sudo apt install pkg-config libssl-dev
```
## Running tests
Some tests require credentials to actual BaCa server.
These can be set using environment variables.
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
