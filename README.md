# hxd: simplistic hexdump processor and editor

## Features

`hxd dump`: dumps the input into a `xxd`-like hexdump
format.

`hxd load`: loads the format produced by `dump`, convert
it back into binary. Note that it currently ignores the
offset part on the left and the ASCII preview part on
the right.

`hxd edit`: summons your `$EDITOR` to edit a binary file
as the hexdump format. Same as above, the offset and
preview part of the dump are currently ignored.

See `hxd [subcommand] --help` for possible flags and
arguments.

## TODO

- [ ] Patch mode: respect the offset and overwrite a
  small portion of target file; support loading sparse
  and not-in-order dump files
- [ ] Octal, binary and decimal dumps

## License

`hxd` is distributed under the terms of the Apache
License 2.0. See [LICENSE](LICENSE) for details.
