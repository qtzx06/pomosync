# pomosync

a terminal based universal pomodoro timer, written in rust.

## the cycle

it's a 2-hour cycle that starts every even hour (12:00, 2:00, 4:00, etc.).

here's how it breaks down:

- **00:00 - 25:00:** work (25 mins)
- **25:00 - 30:00:** short break (5 mins)
- **30:00 - 55:00:** work (25 mins)
- **55:00 - 60:00:** short break (5 mins)
- **60:00 - 85:00:** work (25 mins)
- **85:00 - 90:00:** short break (5 mins)
- **90:00 - 120:00:** long break (30 mins)

when you open it, it just shows you where you are in the current cycle.

## installation

it's built with rust.

### prerequisites

you need the rust toolchain (`rustc` and `cargo`). the install script will yell at you if you don't have it.

### install steps

1.  clone the repo:
    ```bash
    git clone https://github.com/qtzx06/pomosync.git
    cd pomosync/tui
    ```

2.  run the script:
    ```bash
    ./install.sh
    ```

it builds the app and sticks the binary in `~/.local/bin/pomosync`. make sure that directory is in your `path`.

## usage

just type `pomosync` in your terminal.

### controls

-   `(q)`: quit
-   `(p)`: pause
-   `(m)`: menu

## releases

ima add more features ong! and .config stuff in the future.