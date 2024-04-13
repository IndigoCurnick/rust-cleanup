# Rust Cleanup

!!!THIS SOFTWARE MAKES IRREVERSIBLE CHANGES TO YOUR COMPUTER BY DELETING FILES!!!

!!!PLEASE THINK CAREFULLY BEFORE RUNNING THIS SCRIPT!!!

Do YOUR Rust projects hog disk space? 
Do YOU hate deleting the target folders manually?
Look no further than `rust-cleanup`!

Rust `target` folders can become HUGE - my website's target folder is over 4GB. 
You can easily accumulate over 100Gb (or more...) with several large projects,
many of which might just sit on the disc for a long time doing nothing.

`rust-cleanup` will search from your current working directory and delete 
all `target` folders next to `Cargo.toml` files.

It will also report on the disk space freed.

Simply compile with 

```
cargo build --release
```

And then run the binary with `./rust-cleanup`