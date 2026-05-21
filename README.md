# ftidy

[中文文档](README_zh.md)

File tidying tool: deduplicate and batch rename.

## Install

```sh
cargo install --path .
```

## Usage

### dedup — Find and remove duplicate files

```sh
# Dry-run: show duplicates without deleting
ftidy dedup ~/Photos

# Actually delete duplicates
ftidy dedup ~/Photos --delete
```

Compares files by SHA-256 content hash. Keeps the first occurrence, reports (or removes) the rest.

### rename — Batch rename files

```sh
ftidy rename *.jpg -p "vacation_{n}"
```

Template variables:
- `{n}` — sequence number (001, 002, ...)
- `{name}` — original filename (without extension)
- `{ext}` — original extension

If the pattern doesn't contain a `.`, the original extension is preserved.

## Yazi Integration

ftidy ships with a [Yazi](https://yazi-rs.github.io) plugin for in-file-manager dedup and rename.

### Install plugin

```sh
# Copy plugin to yazi config
cp -r yazi-plugin/ftidy.yazi ~/.config/yazi/plugins/
```

### Keybindings

Add to your `~/.config/yazi/keymap.toml`:

```toml
[[mgr.prepend_keymap]]
on  = ["f", "d"]
run = "plugin ftidy dedup"
desc = "ftidy: deduplicate files in current directory"

[[mgr.prepend_keymap]]
on  = ["f", "r"]
run = "plugin ftidy rename"
desc = "ftidy: batch rename selected files"
```

## License

MIT
