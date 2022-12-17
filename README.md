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
- VMT Hooking (paint + painttraverse example)
- Engine rendering (drawrect example in painttraverse)

## Bugs/TODO
- Gameoverlay hooking seems to mess up if you unload and inject again, not sure why
- Messy code in some places as im still learning proper rust game hacking practices (if they even exist yet) while writing this
- Could be optimized further
- TODO: Remove the random debug logging
- TODO: Add gameoverlayrenderer dx9 rendering 
- TODO: Actually implement example features
- TODO: Config system to go with above todo
