#### Hellsgate mini File Manager

This repository is an exploration of the [Hellgate](https://github.com/am0nsec/HellsGate) technique developed by @am0nsec and @RtlMateusz.
It is based on the two implementations:
- [rust_syscalls](https://github.com/janoglezcampos/rust_syscalls)
- [Rust-Hells-Gate](https://github.com/0xflux/Rust-Hells-Gate)

It wraps basic `Nt*` functions on top of the Hellsgate technique, with inderect syscalls. 

For reference, in the [rust_syscalls](https://github.com/janoglezcampos/rust_syscalls/blob/162451aaf095c8cb8c5e6b33ebf0bf44c62aca34/src/syscall.rs#L34) project, extra parameters are handled explicitly inside the syscall functions, providing a more dynamic way of supporting different argument counts. The current implementation takes a more explicit and straightforward approach, adding `Nt*` function implementations in `src/syscalls.asm`.
