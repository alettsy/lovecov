<div align="center">
    <img width="250" src="docs/logo.png" />
</div>

<div align="center">
    <i>A simple, cross-platform lcov parser for generating various output formats.</i>
</div>

<br />

<div align="center">
    <a href="https://crates.io/crates/lovecov">
      <img src="https://img.shields.io/crates/v/lovecov" alt="version">
    </a>    
    <a href="https://github.com/alettsy/lovecov/issues">
      <img src="https://img.shields.io/github/issues/alettsy/lovecov" alt="open issues">
    </a>    
</div>

<div align="center">
    <a href="https://ko-fi.com/W7C027ZZTS">
        <img src="https://ko-fi.com/img/githubbutton_sm.svg" alt="ko-fi" />
    </a>
</div>

<hr />

## Installation

### Cargo

The easiest way to install it is via `cargo`:

```bash
cargo install lovecov
```

### Manual build

To manually build it, you can simply do:

```bash
git clone https://github.com/alettsy/lovecov
cd lovecov
cargo build
```

The output should end up in the `target` folder.

## Usage

The `lovecov` CLI is pretty straightforward:

```bash
lovecov <file> <output (optional)>
```

If you do not provide an `output`, it will default to a simple summary printed in the terminal.

All outputs currently supported:

1. `summary` - simple printed summary
2. `html` - single, static HTML file with navigatable tabs

### Example

```bash
lovecov my_lcov.info html
```

This will output `coverage.html` to the current directory.

## Screenshots

### Summary

![Summary](docs/summary.png)

### HTML

![HTML](docs/html.png)

## Contributing

All contributions are welcome!

See [CONTRIBUTING](CONTRIBUTING.md) for how.

## Planned features

- More output formats, such as CSV
- Utilities options, such as omitting files from outputs
- `.lovecov` file to make setup easier
