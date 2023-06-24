## what is this
rust with plugins
## how does this work?
idk it's magic
## how to run this
build the example plugin 
```
cargo build -p example_plugin
```
move the libmy_plugin.so to the plugins folder
```
cp target/debug/libmy_plugin.so plugins/
```
run the program
```
cargo run
```