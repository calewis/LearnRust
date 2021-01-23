# Learn Rust

One of my goals for the new year is to learn rust. This is a place for me to
experiment with that. 

## Cross Compile for Raspberry Pi 4

First need to add an Arm rust target that works on the Rapsberry Pi
```sh
rustup target add armv7-unknown-linux-musleabihf
```

Then we need to add the arm linker to the local `.cargo/config.toml` file
```toml
[target.armv7-unknown-linux-musleabihf]
linker = "arm-linux-gnueabihf-ld"
```
---
**NOTE**
I have tried to get the gnueabihf rust target working, but so far no luck
---

#### What is MUSL

> [musl](https://musl.libc.org/) is an implementation of the C standard library
> built on top of the Linux system call API, including interfaces defined in
> the base language standard, POSIX, and widely agreed-upon extensions.

It is nice because you can statically link it into your rust program.

### Get Arm cross compiler tools
```sh
brew install arm-linux-gnueabihf-binutils 
```
which gives us
```sh
> which arm-linux-gnueabihf-ld
/usr/local/bin/arm-linux-gnueabihf-ld
```


### ~Building GCC from Source~ doesn't work yet

get gcc source

```sh
git clone git://gcc.gnu.org/git/gcc.git
```

Generate configure for gcc
```sh
cd gcc
autoreconf --verbose
```
---
**NOTE**
I had to use `brew install automake` to get aclocal
---

Move to a build directory and run

