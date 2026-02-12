# Guide: Setting Up the Korlang Package Registry (K-Registry)                                                                             │
│  2                                                                                                                                           │
│  3 This guide explains how to host a Korlang package registry using **GitHub** for storage and **GitHub Pages** as the static API endpoint.  │
│    This allows for a decentralized, low-cost, and high-availability package ecosystem.                                                       │
│  4                                                                                                                                           │
│  5 ---                                                                                                                                       │
│  6                                                                                                                                           │
│  7 ## 1. Repository Structure                                                                                                                │
│  8 Create a new GitHub repository (e.g., `korlang-registry`). The structure should follow the KPM expectations:                              │
│  9                                                                                                                                           │
│ 10 ```                                                                                                                                       │
│ 11 /                                                                                                                                         │
│ 12 ├── index.json          # Global index of all available packages                                                                          │
│ 13 ├── packages/                                                                                                                             │
│ 14 │   ├── ui-kit/                                                                                                                           │
│ 15 │   │   ├── metadata.json                                                                                                                 │
│ 16 │   │   └── v1.0.0.tar.gz                                                                                                                 │
│ 17 │   └── network-lib/                                                                                                                      │
│ 18 │       ├── metadata.json                                                                                                                 │
│ 19 │       └── v2.1.0.tar.gz                                                                                                                 │
│ 20 └── authors/                                                                                                                              │
│ 21     └── user_id.json    # Author verification and public keys                                                                             │
│ 22 ```                                                                                                                                       │
│ 23                                                                                                                                           │
│ 24 ---                                                                                                                                       │
│ 25                                                                                                                                           │
│ 26 ## 2. Setting Up GitHub Pages                                                                                                             │
│ 27 1. Go to your repository **Settings** -> **Pages**.                                                                                       │
│ 28 2. Select the `main` branch (or `/root` folder) as the source.                                                                            │
│ 29 3. Once deployed, your registry API will be live at `https://<user>.github.io/korlang-registry/`.                                         │
│ 30                                                                                                                                           │
│ 31 ---                                                                                                                                       │
│ 32                                                                                                                                           │
│ 33 ## 3. The `index.json` Format                                                                                                             │
│ 34 KPM reads this file to resolve package names to their metadata locations.                                                                 │
│ 35 ```json                                                                                                                                   │
│ 36 {                                                                                                                                         │
│ 37   "packages": {                                                                                                                           │
│ 38     "ui-kit": "packages/ui-kit/metadata.json",                                                                                            │
│ 39     "network-lib": "packages/network-lib/metadata.json"                                                                                   │
│ 40   }                                                                                                                                       │
│ 41 }                                                                                                                                         │
│ 42 ```                                                                                                                                       │
│ 43                                                                                                                                           │
│ 44 ---                                                                                                                                       │
│ 45                                                                                                                                           │
│ 46 ## 4. Package Metadata (`metadata.json`)                                                                                                  │
│ 47 Each package folder contains a metadata file describing versions and checksums.                                                           │
│ 48 ```json                                                                                                                                   │
│ 49 {                                                                                                                                         │
│ 50   "name": "ui-kit",                                                                                                                       │
│ 51   "owner": "korlang-dev",                                                                                                                 │
│ 52   "versions": {                                                                                                                           │
│ 53     "1.0.0": {                                                                                                                            │
│ 54       "dist": "v1.0.0.tar.gz",                                                                                                            │
│ 55       "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",                                                       │
│ 56       "dependencies": {                                                                                                                   │
│ 57         "core": "^1.0.0"                                                                                                                  │
│ 58       }                                                                                                                                   │
│ 59     }                                                                                                                                     │
│ 60   }                                                                                                                                       │
│ 61 }                                                                                                                                         │
│ 62 ```                                                                                                                                       │
│ 63                                                                                                                                           │
│ 64 ---                                                                                                                                       │
│ 65                                                                                                                                           │
│ 66 ## 5. Publishing to Your Registry                                                                                                         │
│ 67 To publish a package, KPM will perform the following via the `kpm publish` command (once configured):                                     │
│ 68 1. Bundle the Korlang source into a `.tar.gz`.                                                                                            │
│ 69 2. Generate a SHA256 checksum.                                                                                                            │
│ 70 3. Use the **GitHub API** to upload the tarball to the `packages/` directory.                                                             │
│ 71 4. Update `metadata.json` and `index.json` via a automated GitHub Action or direct API commit.                                            │
│ 72                                                                                                                                           │
│ 73 ---                                                                                                                                       │
│ 74                                                                                                                                           │
│ 75 ## 6. Configuring KPM to Use Your Registry                                                                                                │
│ 76 In your local `Korlang.config` or global KPM settings, add your registry URL:                                                             │
│ 77                                                                                                                                           │
│ 78 ```korlang                                                                                                                                │
│ 79 [registry]                                                                                                                                │
│ 80 default = "https://<user>.github.io/korlang-registry/"                                                                                    │
│ 81 ```                                                                                                                                       │
│ 82                                                                                                                                           │
│ 83 ---                                                                                                                                       │
│ 84                                                                                                                                           │
│ 85 ## 7. Security (Author Verification)                                                                                                      │
│ 86 KPM uses GPG signing for packages.                                                                                                        │
│ 87 - The `authors/` directory in the registry contains public keys.                                                                          │
│ 88 - KPM downloads the key associated with the package owner to verify the tarball signature before extraction.                              │
│ 89                                                                                                                                           │
│ 90 **Tip:** Use GitHub Actions to automatically rebuild the `index.json` whenever a new package is uploaded to the `packages/` folder.      