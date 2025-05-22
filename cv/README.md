# How to use
0. Install [rust](https://www.rust-lang.org/tools/install)
1. Install llvm and opencv
```zsh
brew install llvm
brew install opencv
```
2. set up your ~/.zshrc (or equivalent)
```zsh
export LLVM_PATH="$(brew --prefix llvm)"
export PATH="$LLVM_PATH/bin:$PATH"
export DYLD_LIBRARY_PATH="$LLVM_PATH/lib:$DYLD_LIBRARY_PATH"
```
3. `cargo run`

This is assuming a 1920*1080 webcam, will add adjustments later but for now if that's not you, adjust the ROW_LEN and COL_LEN consts