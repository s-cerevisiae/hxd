# hxd: hex editor at home

`hxd` is a simplistic command-line utility that can be used to view, produce or
edit a binary file, in a format inspired by `xxd`. See the documentation below
for details.

## Features

### `hxd dump`

Dumps the input into [the hxd format](#the-hxd-format).

```sh
> cat 1.txt
abcdefgh
> hxd dump 1.txt
00000000: 61626364 65666768 0a | abcdefgh.
```

### `hxd load`
Loads the input in hxd format, convert it back into binary. The offset and
comments part are ignored and can be omitted.

```sh
> cat 1.hxd
00000000: 61626364 65666768 0a | abcdefgh.
> hxd load 1.hxd
abcdefgh
```

### `hxd edit`
Summons your `$EDITOR` to edit a binary file as the hxd format. Same as above,
the offset and comment part of the dump are ignored on save. Note that the
whole file will be dumped into the editor, so use it with care on large files.

### `hxd patch`
Reads input from stdin and apply it as a patch to the target file,
**respecting** offset. A line without offset continues the last line.

```sh
> hxd dump 1.txt
00000000: 61626364 65666768 0a | abcdefgh.
> echo '01:72' | hxd patch 1.txt
> cat 1.txt
arcdefgh
```

See `hxd [subcommand] --help` for possible flags and arguments.

## The hxd format

The hxd format is a plaintext format for expressing arbitrary byte string
accompanied with numeric labels.

### Syntax

A file in hxd format consists of multiple lines.  Each line can have three
parts in them: `offset`, `data` and `comment`.

- `offset` is a hexadecimal number followed by `:`. It can also start with `+`
  or `-`. Examples: `00000000` `+a1` `-5`
- `data` is a string of hexadecimal digits, where each pair of two digits form
  an octet. Octets can be separated with ascii whitespace characters. Examples:
  `5f3759df` `de ad beef` `ABCD`
- `comment` is an arbitrary string prefixed with `|`. It extends till the end
  of current line. Example: `| this is a comment`

`offset` and `comment` are optional, and `data` can be empty.

```
<hxd> := {<line> <eol>}
<line> := [<offset> ":"] <data> ["|" <comment>]
<offset> := /[+-]?[0-9a-fA-F]+/
<data> := {/[0-9a-fA-F]{2}/ " "?}
<comment> := /.*/
```

### Semantics and behavior

In the `hxd` command line tool, the hxd format is interpreted in the following way:

`data` is interpreted as bytes corresponding to the octets, and the whitespaces
between them are ignored. For example, `6162 6364` is interpreted as the ascii
string `abcd`.

`offset` is ignored in `hxd load` and `hxd edit`. In `hxd patch`, `offset`
represents the starting byte position of the following `data` that is going to
be written into the target file. There are two kind of offsets: absolute and
relative ones.
- An offset without any prefix is an absolute offset, counting from the first
  byte of a file as `0`.
- An offset stating with `+` or `-` is a *relative* offset, which is counted
  from **the last offset** `hxd` has encountered. For example: in `00:
  aabbccdd\n+01: 99`, the byte `99` will be written to the byte at absolute
  offset `01`. Applying the patch to an empty file with `hxd patch` will yield
  `00: aa99ccdd` (binary file shown in hxd format).

In the output of `hxd dump` and `hxd edit`, the data section is grouped with
space and each line is broken at the given length. Offsets are always absolute.
Comments are used as a "preview" of the data on the left, with ASCII printable
characters shown, and others replaced with `.`.

## TODO

- [x] Patch mode: respect the offset and overwrite a small portion of target
  file; support loading sparse and not-in-order dump files
- [ ] More examples on usage; demo in asciinema
- [ ] More sophisticated cli options: `--offset`, `hxd edit --patch`, etc.
- [ ] Octal and binary dumps

## License

`hxd` is distributed under the terms of the Apache License 2.0. See
[LICENSE](LICENSE) for details.
