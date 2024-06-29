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

### `hxd dump`

Dumps the input into the hxd format.

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

## TODO

- [x] Patch mode: respect the offset and overwrite a small portion of target
  file; support loading sparse and not-in-order dump files
- [ ] More sophisticated cli options: `--offset`, `hxd edit --patch`, etc.
- [ ] Octal and binary dumps

## License

`hxd` is distributed under the terms of the Apache License 2.0. See
[LICENSE](LICENSE) for details.
