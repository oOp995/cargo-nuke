# 💥 cargo-nuke V0.1.4

> *If you're here, you're probably running out of storage.  
> `cargo-nuke` is here to give you a hand.*

## install
```bash
cargo install cargo-nuke
``` 

## use
### - run with default settings
- this will run with defaults [  `--older-than 30` ,`--path current` ]
and **requires confirmation** . 
    
```
cargo nuke 
```
or
```
cargo-nuke
```
### - specify path and period
* this will configure to clean all artifacts older than  `40` days in path `~/rust-projects`, **requires confirmation**.
```
cargo nuke -p ~/rust-projects --older-than 40
```
or
```
cargo nuke --older-than 40 --path ~/rust-projects
```

### - `dry-run`
```
cargo nuke --dry-run -p ~/rust-projects --older-than 39
```
or simply `dry-run` in place
```
cargo nuke --dry-run
```

# Advanced zone **proceed with caution**
activating `--sure` flag will **bypass confirmation** and nuke all build artifacts in the specefied `--path` and `--older-than`

in place with defaults
```
cargo nuke --sure
```

in path with configs
```
cargo nuke --sure --path ~/rust-projects --older-than 40
```

#### ⚠️  "Some people just want to watch the world burn"
```
cargo nuke --sure --path ~/rust-projects --older-than 0
```
#### 💥 37.2 GB freed

## Conflicts
* `--sure` with `dry-run` is a conflict .
## Changelog
* see [CHANGELOG.MD](/CHANGELOG.MD) 


## help
```
cargo nuke --help
```

## v 1.0.0 Roadmap
*   better semantics to determine last modified/use of crate closer to `cargo-fingerprint` owns . 

* better search criteria .
* exclude flags and much more. 
* better error handling .


## Safety guarantees
>  `cargo-nuke` guarantes
* safe target deletes, refuse to delete targets in any folder not linked to rust project .
* safe against symlinks attacks/misuse .
* it do not touch cache or global files .
* it refuses to delete symbolic links folders or files in favor of **safety**
 