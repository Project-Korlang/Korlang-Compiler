# KPM Lockfile Schema (Draft)

This document defines the initial `KPM.lock` schema for Korlang dependencies.

## Format
The lockfile is TOML.

## Schema
```
[package]
name = "my-app"
version = "0.1.0"

[[dependency]]
name = "serde"
source = "registry"
version = "1.0.198"
checksum = "sha256:..."

[[dependency]]
name = "mylib"
source = "git"
repo = "https://example.com/mylib.git"
rev = "a1b2c3d4"
checksum = "sha256:..."

[[dependency]]
name = "local-crate"
source = "path"
path = "../local-crate"
checksum = "sha256:..."

[[dependency]]
name = "numpy"
source = "pip"
version = "1.26.4"
checksum = "sha256:..."

[[dependency]]
name = "opencv"
source = "cpp"
version = "4.9.0"
checksum = "sha256:..."

