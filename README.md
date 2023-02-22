# key-stroke-counter

Simple program that counts keystrokes. Most of the program is taken from <https://github.com/gsingh93/keylogger> but is modified to simply count the number of keystrokes instead of actual input.

## Installation

Clone the repository:

```
$ git clone git@github.com:juliarose/key-stroke-counter.git
$ cd key-stroke-counter
```

Build the code:

```$ cargo build --release```

You can run the code with Cargo or directly from the target directory. Note that the counter must be run as the root user:

```
$ sudo cargo run --release -- -h
$ sudo ./target/release/key-stroke-counter -h
```

You can move the `key-stroke-counter` binary wherever you want. For example, you can put it in `/usr/local/bin` or in any other directory in your path.

## Usage

```
$ sudo cargo run -- -h

Usage: target/release/key-stroke-counter [options]

Options:
    -h --help           prints this help message
    -v --version        prints the version
    -d --device DEVICE  specify the device file
    -f --file FILE      specify the file to log to
```

If the `-f` flag is not specified, the file `keys.log` is used.

If you would like to run the counter in the background, append an `&` to the end of the command. If you would like to run the counter as a daemon or at startup, use init script/service manager that comes with your distro. An example `systemd` file is provided.

## License

[MIT](https://github.com/juliarose/key-stroke-counter/blob/master/LICENSE)