# sway-vim-kbswitch &emsp; [![Version Badge]][crates.io] [![License Badge]][license]

[Version Badge]: https://img.shields.io/crates/v/libswaykbswitch.svg
[crates.io]: https://crates.io/crates/libswaykbswitch
[License Badge]: https://img.shields.io/crates/l/libswaykbswitch.svg
[license]: https://github.com/khaser/sway-vim-kbswitch/blob/master/LICENSE.md

A Rust shared library that provides the interface for Vim keyboard layout switch [plugin](https://github.com/lyokha/vim-xkbswitch).

## Usage
- Install [vim-xkbswitch](https://github.com/lyokha/vim-xkbswitch) plugin in your vim
- Compile this library using cargo or install as a package.
```
git clone https://github.com/khaser/sway-vim-kbswitch && \
cd sway-vim-kbswitch && cargo build -r
```
[![Packaging status](https://repology.org/badge/vertical-allrepos/libswaykbswitch.svg)](https://repology.org/project/libswaykbswitch/versions)

- Write the path to the library to the variable `g:XkbSwitchLib` in your .vimrc settings.
```
let g:XkbSwitchEnabled = 1
let g:XkbSwitchLib = '<PathToBuildDir>/target/release/libswaykbswitch.so'
```

#### Optional (f and r action)
For using f and r action with last layout for insert mode you can put following lines into your vimrc:
```
let g:XkbSwitchAssistNKeymap = 1
set keymap=russian-jcukenwin
let g:XkbSwitchKeymapNames = {'Russian' : 'ru'}
```

## Troubleshooting
When you run vim with sudo command you should preserve environment variable ``SWAYSOCK``.
Example of run with sudo:
```
sudo --preserve-env=SWAYSOCK vim
```
You can put following line into you sudoers file to fix this problem:
```
Defaults  env_keep += "SWAYSOCK"
```

