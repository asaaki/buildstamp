# builstamp

Returns a buildstamp, like `23W42.12345`

Useful for tagging builds and images.

This replaces my previous snippet with the date command in Makefiles:

```makefile
STAMP = $(shell bash -c 'date +%gW%V.%w%H%M')
```

This is a highly opinionated tool.
You might not need it.

## Installation

`cargo install buildstamp -F cli --locked`

## Usage

```sh
buildstamp
#=> 23W42
# a short stamp with year and week only
# like Minecraft snapshots

buildstamp minute
#=> 23W42.12345
# has <week day numer><hour><minute> added to it

buildstamp --help
# for more options
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
