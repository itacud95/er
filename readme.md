
# Building

From repo root: 
```
cargo install --force --path .
```
Look through the output to find the completions file: 
```
Replacing /home/jk/.cargo/bin/er
```
Then install the completion: 
```
complete -C /home/jk/.cargo/bin/er er
```
Add binary to path: 
```
PATH=$PATH:/home/jk/dev/er/target/release
```
