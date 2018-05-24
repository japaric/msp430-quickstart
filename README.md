# `msp430-quickstart`

# Quick start for Debian/Ubuntu Linux

* Install toolchain: [msp430-gcc](http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSPGCC/latest/index_FDS.html) (at May 2018 it is msp430-gcc-6.4.0.32)
* Install mspdebug: `apt install mspdebug`
* Install rust-nightly: `rustup install nightly`
* Install rust-src: `rustup component add rust-src`
* Clone msp430-quickstart: `git clone https://github.com/japaric/msp430-quickstart.git && cd msp430-quickstart`
* Build project: `xargo rustc --target msp430-none-elf --release`
* Program Launchpad: ` mspdebug rf2500 'prog target/msp430-none-elf/release/msp430-quickstart'`


# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
