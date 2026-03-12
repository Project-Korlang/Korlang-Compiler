#!/usr/bin/env bash
REGISTRY_DIR="$HOME/.cargo/registry/src/index.crates.io-6f17d22bba15001f"
VENDOR_DIR="/mnt/c/Users/nanda/Desktop/KUBUNTU/Korlang/Korlang-Compiler/vendor"

mkdir -p "$REGISTRY_DIR"

cd "$VENDOR_DIR"
for d in *; do
    if [ -d "$d" ]; then
        # Handle versions if they are not in the folder name
        # Cargo expects folder names like "inkwell-0.8.0"
        # Since cargo vendor might produce just "inkwell", we check Cargo.toml
        VERSION=$(grep '^version =' "$d/Cargo.toml" | head -1 | cut -d '"' -f 2)
        TARGET_NAME="${d}-${VERSION}"
        echo "Copying $d as $TARGET_NAME"
        mkdir -p "$REGISTRY_DIR/$TARGET_NAME"
        cp -r "$d"/* "$REGISTRY_DIR/$TARGET_NAME/"
    fi
done
