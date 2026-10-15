[windows]
set shell := ["powershell.exe", "-c"]

[linux]
set shell := ["bash", "-c"]

alias bc := before_commit

before_commit:
    cargo test
    cargo fmt -- --check
    cargo clippy -- -D warnings
