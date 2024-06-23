# hxd: non-interactive hexdump processor

`hxd` is a simplistic command-line utility that can be used to view, produce or
edit a binary file, in the following format:
```
<offset>: <hexdump> | <comments>
```

For example:
```
00000000: 68657864 756d700a | hexdump.
```
where `offset` is the number `00000000`(hex), `hexdump` is the hexadecimal form
of the ASCII string `"hexdump\n"`, and `comments` are a preview of the string
with ASCII printable characters shown as-is and others shown as `.`.

## Features

`hxd dump`: dumps the input into aforementioned hexdump format.

`hxd load`: loads the format produced by `dump`, convert it back into binary.
The offset and comments part are ignored and can be omitted.

`hxd edit`: summons your `$EDITOR` to edit a binary file as the hexdump format.
Same as above, the offset and comment part of the dump are ignored when saving.

See `hxd [subcommand] --help` for possible flags and arguments.

## TODO

- [ ] Patch mode: respect the offset and overwrite a small portion of target
  file; support loading sparse and not-in-order dump files
- [ ] Octal and binary dumps

## License

`hxd` is distributed under the terms of the Apache License 2.0. See
[LICENSE](LICENSE) for details.
