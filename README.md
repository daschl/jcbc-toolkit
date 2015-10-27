JCBC Toolkit
============

This program is a heavy work in progress, move on unless you want to learn some rust or desperately need this program for some reason.

Usage
-----

Of course there is no binary version released yet, so you need to build it from source. Thankfully, rust makes it easy. It builds against rust 1.3, so make sure you have at least this version installed.

Then cloning and running is all you need. Don't forget to point it to a logfile for now like this (args are subject to change):

```
$ git clone https://github.com/daschl/jcbc-toolkit.git
$ cd jcbc-toolkit
$ cargo run mylogfile.log
```

If you encounter a weird panic, please run it like this and report an issue:

```
RUST_BACKTRACE=1 cargo run mylogfile.log
```

The actual binary is called `jt` and can be found under `target/debug/jt` or if built with `cargo build --release` under `target/release/jt`.

Contributing
------------
Source code quality is subpar right now, since I'm learning rust and dogfooding the project at the same time. There is lots of stuff which for sure can be cleaned up - feel free to help out.