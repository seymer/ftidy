# ftidy

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

## License

MIT
