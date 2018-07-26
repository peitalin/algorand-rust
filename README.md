# algorand-rust

Demo implementation of Algorand Agreement (2018)
https://eprint.iacr.org/2018/377.pdf

Demo:
```
git clone https://github.com/peitalin/algorand-rust;
cd algorand-rust;
cargo run values;
cargo run nullvotes;
```

Todo:
- peers and network responses
- VRFs

Directory Structure:
```
"src"
├── "config"
│   └── "mod.rs"
├── "main.rs"
└── "votes"
    ├── "max_hash_map.rs"
    ├── "message_type.rs"
    ├── "mod.rs"
    ├── "signatures.rs"
    └── "vote.rs"
```
