# nxd: hex editor at home

`nxd` is a simplistic command-line utility that can be used to view, produce or
edit a binary file, in a format inspired by `xxd`. See the documentation below
for details.

## Features

### `nxd dump`

Dumps the input into [the nxd format](#the-nxd-format).

```sh
> cat 1.txt
abcdefgh
> nxd dump 1.txt
00000000: 61626364 65666768 0a | abcdefgh.
```

### `nxd load`
Loads the input in nxd format, convert it back into binary. The offset and
comments part are ignored and can be omitted.

```sh
> cat 1.nxd
00000000: 61626364 65666768 0a | abcdefgh.
> nxd load 1.nxd
abcdefgh
```

### `nxd edit`
Summons your `$EDITOR` to edit a binary file as the nxd format. Same as above,
the offset and comment part of the dump are ignored on save. Note that the
whole file will be dumped into the editor, so use it with care on large files.

### `nxd patch`
Reads input from stdin and apply it as a patch to the target file,
**respecting** offset. A line without offset continues the last line.

```sh
> nxd dump 1.txt
00000000: 61626364 65666768 0a | abcdefgh.
> echo '01:72' | nxd patch 1.txt
> cat 1.txt
arcdefgh
```

See `nxd [subcommand] --help` for possible flags and arguments.

## The nxd format

The nxd format is a plaintext format for expressing arbitrary byte string
accompanied with numeric labels.

### Syntax

A file in nxd format consists of multiple lines.  Each line can have three
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
<nxd> := {<line> <eol>}
<line> := [<offset> ":"] <data> ["|" <comment>]
<offset> := /[+-]?[0-9a-fA-F]+/
<data> := {/[0-9a-fA-F]{2}/ " "?}
<comment> := /.*/
```

### Semantics and behavior

In the `nxd` command line tool, the nxd format is interpreted in the following way:

`data` is interpreted as bytes corresponding to the octets, and the whitespaces
between them are ignored. For example, `6162 6364` is interpreted as the ascii
string `abcd`.

`offset` is ignored in `nxd load` and `nxd edit`. In `nxd patch`, `offset`
represents the starting byte position of the following `data` that is going to
be written into the target file. There are two kind of offsets: absolute and
relative ones.
- An offset without any prefix is an absolute offset, counting from the first
  byte of a file as `0`.
- An offset stating with `+` or `-` is a *relative* offset, which is counted
  from **the last offset** `nxd` has encountered. For example: in `00:
  aabbccdd\n+01: 99`, the byte `99` will be written to the byte at absolute
  offset `01`. Applying the patch to an empty file with `nxd patch` will yield
  `00: aa99ccdd` (binary file shown in nxd format).

In the output of `nxd dump` and `nxd edit`, the data section is grouped with
space and each line is broken at the given length. Offsets are always absolute.
Comments are used as a "preview" of the data on the left, with ASCII printable
characters shown, and others replaced with `.`.

## TODO

- [x] Patch mode: respect the offset and overwrite a small portion of target
  file; support loading sparse and not-in-order dump files
- [ ] More examples on usage; demo in asciinema
- [ ] More sophisticated cli options: `--offset`, `nxd edit --patch`, etc.
- [ ] Octal and binary dumps

## License

`nxd` is distributed under the terms of the Apache License 2.0. See
[LICENSE](LICENSE) for details.
