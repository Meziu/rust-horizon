# `armv6k-nintendo-3ds`

**Tier: 3**

The Nintendo 3DS platform, which has an ARMv6K processor, and its associated
operating system (`horizon`).

## Target maintainers

- [@Meziu](https://github.com/Meziu)
- [@AzureMarker](https://github.com/AzureMarker)
- [@ian-h-chamberlain](https://github.com/ian-h-chamberlain)

## Requirements

This target is cross-compiled. `#![no_std]` crates should work without much
additional effort.

`std` is partially supported, but mostly works. Some APIs are unimplemented
and will simply return an error, such as `std::process`. An allocator is provided
by default.

In order to support `std` APIs, binaries must be linked against the open-source
[devkitARM toolchain](https://devkitpro.org/wiki/Getting_Started), notably
using its `arm-none-eabi-gcc` as the linker.

Additionally, some helper crates provide implementations of some `libc` functions
use by `std`. These, or an alternate implementation of the relevant functions,
are required to use `std`:

- [`pthread-3ds`](https://github.com/Meziu/pthread-3ds) provides pthread APIs for `std::thread`.
- [`linker-fix-3ds`](https://github.com/Meziu/rust-linker-fix-3ds) fulfills some other missing libc APIs.

Binaries built for this target should be compatible with most variants of the
3DS (and 2DS) hardware and firmware, but testing is limited and some versions may
not work correctly.

This target generates binaries in the ELF format.

## Building the target

You can build Rust with support for the target by adding it to the `target`
list in `config.toml` and providing paths to the devkitARM toolchain, e.g.:

```toml
[build]
build-stage = 1
target = ["armv6k-nintendo-3ds"]

[target.armv6k-nintendo-3ds]
cc = "/opt/devkitpro/devkitARM/bin/arm-none-eabi-gcc"
cxx = "/opt/devkitpro/devkitARM/bin/arm-none-eabi-g++"
ar = "/opt/devkitpro/devkitARM/bin/arm-none-eabi-ar"
ranlib = "/opt/devkitpro/devkitARM/bin/arm-none-eabi-ranlib"
linker = "/opt/devkitpro/devkitARM/bin/arm-none-eabi-gcc"
```

## Building Rust programs

Rust does not yet ship pre-compiled artifacts for this target.

The recommended way to build binaries is by using the
[cargo-3ds](https://github.com/Meziu/cargo-3ds) tool, which uses `build-std`
and provides commands that work like the usual `cargo run`, `cargo build`, etc.

You can also build Rust with the target enabled (see "Building the target" above).

As mentioned in in "Requirements", programs that use `std` must link against
both the devkitARM toolchain and libraries providing the `libc` APIs used in `std`.
There is a general-purpose utility crate for working with nonstandard
APIs provided by the OS: [`ctru-rs`](https://github.com/Meziu/ctru-rs).
Add it to Cargo.toml to use it in your program:

```toml
[dependencies]
ctru-rs = { git = "https://github.com/Meziu/ctru-rs.git" }
```

Using this library's `init()` function ensures the symbols needed to link
against `std` are present (as mentioned in "Requirements" above), as well as
providing a runtime suitable for `std`:

```rust
fn main() {
    ctru::init();
}
```

## Testing

Binaries built for this target can be run in an emulator (most commonly
[Citra](https://citra-emu.org/)), or sent to a device through
the use of a tool like devkitARM's `3dslink`.

The `cargo-3ds` tool mentioned in "Building Rust programs" supports the use of
`3dslink` with `cargo 3ds run`. The default Rust test runner is not supported, but
[custom test frameworks](https://doc.rust-lang.org/beta/unstable-book/language-features/custom-test-frameworks.html)
can be used with `cargo 3ds test` to run unit tests on a device.

The Rust `library/std` test suite is not yet supported.

## Cross-compilation toolchains and C code

C code can be built for this target using the aforementioned devkitARM toolchain.
