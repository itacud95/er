
# Building

From repo root: 
```
cargo install --force --path .
```
Look through the output to find the completions file: 
```
Replacing ~/.cargo/bin/er
```
Then install the completion: 
```
complete -C ~/.cargo/bin/er er
```
Add binary to path: 
```
PATH=$PATH:~/dev/er/target/release
```
