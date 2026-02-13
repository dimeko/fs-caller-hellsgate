use std::env;
use cc::Build;

fn main() {
    Build::new()
        .file(".\\src\\syscalls.asm")
        .compile("syscalls");
}