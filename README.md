# Rust wrapper for MOTIS / Nigiri
This is a Rust wrapper for the [Nigiri core](https://github.com/motis-project/nigiri) (currently [this fork](https://github.com/traines-source/nigiri)) of the [MOTIS project](https://motis-project.de). It enables retrieving static and realtime timetable data from the various formats (e.g. GTFS/GTFS-RT) parsed by Nigiri. The `nigiri-sys` crate provides the raw, unsafe bindings, while the `motis-nigiri` crate in the root of this repository provides a safe wrapper with an abstracted API.

## Building
It is currently not possible to completely build these crates in the Rust build lifecycle. You will need Docker and/or the necessary build dependencies installed locally.

1. Run `nigiri-sys/build-docker.sh` once. This currently probably only works on Linux. For other platforms, adapt the `nigiri-sys/build-docker.sh` or try to run `nigiri-sys/build.sh` directly on your machine. If you already have `nigiri` checked out on your machine, you may want to symlink it to the `nigiri-sys` directory or adapt the build script.
2. Afterwards, you should be able to build the Rust crates (`nigiri-sys` and `motis-nigiri`, which depends on the former) or depending crates as usual (`cargo build`). This requires glibc 2.34 and possibly some other C/C++ stdlib dependencies installed on your machine. Alternatively, you can run `build-docker.sh` in the root of this repository. 

## Using
Note that when using the raw `nigiri-sys` bindings you must always call the respective `*_destroy_*` functions when done, otherwise memory will be leaked. When using `motis-nigiri`, this is done automatically when the respective structs are dropped.