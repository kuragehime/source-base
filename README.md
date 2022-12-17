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
- print / Console writer
- Interface list iteration
- Netvar manager
- Detour hooking via gameoverlayrenderer.dll (createmove example)
- VMT Hooking (paint example)
