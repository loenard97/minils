<div align="center">

# minils
A small `ls` clone, with pretty printing

![](https://img.shields.io/github/last-commit/loenard97/minils?&style=for-the-badge&color=F74C00)
![](https://img.shields.io/github/repo-size/loenard97/minils?&style=for-the-badge&color=F74C00)

</div>


## ▶️ Usage
### default output as grid
```sh
$ minils
Cargo.lock  Cargo.toml  LICENSE  README.md  src  target
```

### `-a` include dotfiles
```sh
$ minils -a
.git        .gitignore  Cargo.lock  Cargo.toml  LICENSE     README.md   src         target
```

### `-l` long list
```sh
$ minils -l
Last modified     Size              Name
────────────────────────────────────────────────────
2023/05/22 20:03  16 kB             Cargo.lock
2023/05/22 20:03  333 B             Cargo.toml
2023/05/19 22:11  10 kB             LICENSE
2023/05/20 10:11  384 B             README.md
2023/05/22 19:48                    src
2023/05/19 17:47                    target
```

### `-t` recursive tree
```sh
$ minils -t
├─ Cargo.lock
├─ Cargo.toml
├─ LICENSE
├─ README.md
├─ src
│  ├─ lib.rs
│  ├─ main.rs
│  └─ util.rs
└─ target
```
