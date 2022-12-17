# source-base

Rust stdlib-less and msvcrt-less internal base for source engine games 

## Info

Build with `cargo build -Zbuild-std`

VSCode overrides:
```
"rust-analyzer.checkOnSave.overrideCommand": [
	"cargo",
	"check",
	"-Zbuild-std",
	"--workspace",
	"--message-format=json",
	"--all-targets"
], 
```

## Features
- HeapAlloc
- print / console writer
- Interface list iteration
- VMT Hooking
