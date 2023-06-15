# baca-cli [![Build](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml/badge.svg)](https://github.com/hjaremko/baca-cli/actions/workflows/build.yml) [![codecov](https://codecov.io/gh/hjaremko/baca-cli/branch/master/graph/badge.svg?token=CP9EWDCOMV)](https://codecov.io/gh/hjaremko/baca-cli)

CLI client for the Jagiellonian University's BaCa online judge

![Preview](https://i.imgur.com/xOAHuXk.png)

## Installation

Using `cargo` is recommended. The latest release binaries can be downloaded **[here](https://github.com/hjaremko/baca-cli/releases/latest)**.

#### Cargo
```shell
cargo install --git https://github.com/hjaremko/baca-cli.git
```
#### Linux
```sh
$ curl -Lo baca.zip https://github.com/hjaremko/baca-cli/releases/download/v0.6.0/baca-0.6.0-linux.zip
$ unzip baca.zip
$ sudo install baca /usr/local/bin/
```
#### macOS
```sh
$ curl -Lo baca.zip https://github.com/hjaremko/baca-cli/releases/download/v0.6.0/baca-0.6.0-linux.zip
$ unzip baca.zip
$ sudo cp baca /usr/local/bin/
```
#### Windows
Download the raw binary and place it in your `PATH` ([What is `PATH`?](https://en.wikipedia.org/wiki/PATH_(variable))).

#### Arch Linux (not maintained)

Download the release from [AUR](https://aur.archlinux.org/packages/baca-cli) and install it using your
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

## Usage

```
baca [OPTIONS] [COMMAND]
```

```
Commands:
  init     Initialise the current directory as a BaCa workspace
  details  Get submit details
  refresh  Refresh session, use in case of a cookie expiration
  log      Print the last N (default 3) submits
  tasks    Print available tasks
  submit   Make a submit
  last     Print details of the last submit
  config   Open a editor to edit BaCa configuration
  clear    Remove the whole `.baca` directory
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...    Sets the level of log verbosity
  -u, --no-update     Disable update check
  -U, --force-update  Force update check
  -h, --help          Print help
  -V, --version       Print version

```

### Workspace initialization: `init`

Initializes current directory as BaCa workspace, similar to `git init`. Currently, passwords are stored in **plain
text.**
User will be asked for credentials, if not provided.

```
baca init [OPTIONS]
```

```
Options:
      --host <HOST>          BaCa hostname, ex. mn2020
  -l, --login <LOGIN>        BaCa login
  -p, --password <PASSWORD>  BaCa password
  -h, --help                 Print help
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
**Submits with no comment on the first line (header) will fail. Please include header.** 

- Optional parameter `--task <id>` explicitly sets problem to submit to. Use `baca tasks` to see what ids are available.
- Optional parameter `--zip` will zip given file before submitting. The archive is saved as **`source.zip`**.
- Optional parameter `--rename` will rename file before submitting and zipping.
- Optional parameter `--no-main` will remove main function from C/C++ files before submitting and zipping.
- Optional parameter `--no-polish` will remove non-unicode characters from files before submitting and zipping.
- Optional parameter `--language <language>` explicitly sets input file language.
- Optional parameter `--skip-header` disabled header verification. Use in case of a non-standard header.
- `submit config` opens editor to edit submit config.
- `submit clear` clears saved submit config.

```
Usage: baca submit [OPTIONS] [COMMAND]

Commands:
  config  Open a editor to edit submit config
  clear   Clear saved submit config
  help    Print this message or the help of the given subcommand(s)

Options:
  -t, --task <TASK_ID>       Task id, use 'baca tasks' to see what ids are available, overrides saved task id
  -f, --file <FILE>          A file to submit, overrides saved path
  -l, --language <LANGUAGE>  Task language. Please provide it exactly as is displayed on BaCa
  -r, --rename <NEW_NAME>    Submit input file under different name
  -s, --save                 Save task config. If provided, future 'submit' calls won't require providing task config
  -z, --zip                  Zip files to 'source.zip' before submitting, overrides saved config
      --no-save              Do not ask for save
      --no-main              Remove main function before submitting. Takes effect only on C/C++ files
      --no-polish            Transliterate Unicode strings in the input file into pure ASCII, effectively removing Polish diacritics
      --skip-header          Skip header verification
  -h, --help                 Print help
```

Example:

```
> baca submit -f hello.cpp
✔ Choose task: · [E] Metoda SOR
Submitting hello.cpp to task [E] Metoda SOR (C++ with file support).
```

#### Saving task info

If you don't want to type task info (id and filename) every time you submit, use `--save` flag to save it. Keep
in mind that the config provided through parameters will override saved data. To completely remove saved data
use `baca submit clear`. To disable automatic prompt for save, use `--no-save`.

Example:

```
> baca submit -f hello.cpp -t 5 --save
Submitting hello.cpp to task [E] Metoda SOR (C++ with file support).
> baca submit
Submitting hello.cpp to task [E] Metoda SOR (C++ with file support).
```

### Recent submits: `log`

Prints statuses of a couple of recent submits (default 3). Parameter `-t <task_id>` lets you print logs for a specific
task. Task ID can be found through `baca tasks`.

```
baca log [optional: number, default 3] [optional: -t <task_id>]
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

Prints details of the last submit. Requires workspace to be initialized. Parameter `-t <task_id>` lets you print logs
for a specific task. Task ID can be found through `baca tasks`.

```
baca last [optional: -t <task_id>]
```

Example:

```
> baca last

● [G] Funkcje sklejane - C++ - 2020-05-17 18:53:09 - submit 4334
├─── 100% - 4/4 pts - Ok
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/4334
 ✅ ── test0/0 - Ok
 ✅ ── test1/0 - Ok
 ✅ ── test2/0 - Ok
 ✅ ── test3/0 - Ok
```

### Any submit details: `details`

Prints details of a given submit. Requires workspace to be initialized.

```
baca details <id>
```

Example:

```
> baca details 2904

● [D] Skalowany Gauss - C++ - 2020-04-22 19:20:07 - submit 2904
├─── 89% - 3.58/4 pts - TimeExceeded
└─── https://baca.ii.uj.edu.pl/mn2020/#SubmitDetails/2904
 ✅ ── testy_jawne/test1 - Ok
 ✅ ── testy_jawne/test2 - Ok
 ✅ ── testy_jawne/test3 - Ok
 ✅ ── testy_jawne/test4 - Ok
 ✅ ── testy_jawne/test5 - Ok
 ✅ ── testy_jawne/test6 - Ok
 ✅ ── testy_jawne/test8 - Ok
 ✅ ── testy/test0 - Ok
 ✅ ── testy/test1 - Ok
 ❌ ── testy/test10 - TimeExceeded
 ❌ ── testy/test11 - TimeExceeded
 ✅ ── testy/test2 - Ok
 ✅ ── testy/test3 - Ok
 ✅ ── testy/test4 - Ok
 ✅ ── testy/test5 - Ok
 ✅ ── testy/test6 - Ok
 ✅ ── testy/test7 - Ok
 ✅ ── testy/test8 - Ok
 ✅ ── testy/test9 - Ok
```

### All tasks: `tasks`

Prints all tasks.

```
baca tasks
```

Example:

```
> baca tasks

● Id: 1 - [A] Zera funkcji - 69 OK
● Id: 2 - [B] Metoda Newtona - 58 OK
● Id: 3 - [C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane - 62 OK
● Id: 4 - [D] Skalowany Gauss - 52 OK
● Id: 5 - [E] Metoda SOR - 64 OK
● Id: 6 - [F] Interpolacja - 63 OK
● Id: 7 - [G] Funkcje sklejane - 59 OK
● Id: 8 - A2 - 1 OK
● Id: 9 - B2 - 2 OK
● Id: 10 - C2 - 1 OK
● Id: 11 - D2 - 2 OK
● Id: 12 - E2 - 1 OK
● Id: 13 - F2 - 3 OK
● Id: 14 - G2 - 2 OK
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

Some tests require credentials to a actual BaCa server, which can be set using environment variables. These tests are
disabled by default, but you can try running them with the command `cargo test -- --ignored`.

```
TEST_BACA_LOGIN=<login>
TEST_BACA_PASSWORD=<password>
TEST_BACA_HOST=<host>
```

## Setting log levels

Log levels are configured by a `-v` flag.

- `no flag` - no additional logs
- `-v` - **info**
- `-vv` - **debug**
- `-vvv or more` - **trace**
