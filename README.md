# sway-vim-kbswitch &emsp; [![Version Badge]][crates.io] [![License Badge]][license]

[Version Badge]: https://img.shields.io/crates/v/libswaykbswitch.svg
[crates.io]: https://crates.io/crates/libswaykbswitch
[License Badge]: https://img.shields.io/crates/l/libswaykbswitch.svg
[license]: https://github.com/khaser/sway-vim-kbswitch/blob/master/LICENSE.md

A Rust shared library that provides the interface for Vim keyboard layout switch [plugin](https://github.com/lyokha/vim-xkbswitch).

## Usage 
- Install [vim-xkbswitch](https://github.com/lyokha/vim-xkbswitch).
- Compile this library using cargo.
```
git clone https://github.com/khaser/sway-vim-kbswitch && \
cd sway-vim-kbswitch && cargo build -r
```

- Write the path to the library to the variable `g:XkbSwitchLib` in your .vimrc settings.
```
let g:XkbSwitchEnabled = 1
let g:XkbSwitchLib = '<PathToBuildDir>/target/release/libswaykbswitch.so'
```
## Problems 
When you run vim with sudo command you should preserve environment variable.
``SWAYSOCK``. 
Example of run with sudo:
```
sudo --preserve-env=SWAYSOCK vim
```
You can put sudo alias into you .bashrc to fix this problem.

