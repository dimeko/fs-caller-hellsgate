### Hellsgate mini File Manager

This repository is an exploration of the [Hellgate](https://github.com/am0nsec/HellsGate) technique developed by @am0nsec and @RtlMateusz.
It is based on the two implementations:
- [rust_syscalls](https://github.com/janoglezcampos/rust_syscalls)
- [Rust-Hells-Gate](https://github.com/0xflux/Rust-Hells-Gate)

It wraps basic `Nt*` functions on top of the Hellsgate technique, with inderect syscalls. 

For reference, in the [rust_syscalls](https://github.com/janoglezcampos/rust_syscalls/blob/162451aaf095c8cb8c5e6b33ebf0bf44c62aca34/src/syscall.rs#L34) project, extra parameters are handled explicitly inside the syscall functions, providing a more dynamic way of supporting different argument counts. The current implementation takes a more explicit and straightforward approach, adding `Nt*` function implementations in `src/syscalls.asm`.

**NOTE**: Still under development.

#### Small quest

I am encountering an unexpected (at least to me) behaviour in `HFile::write(...)` method. If I put the `syscall_addr` and `ssn` value change right before the `hNtWriteFile` call, I get an error:
```rust
// Runtime error!!!
unsafe {
    defs::hNtWriteFileSyscallAddr = syscall_addr;   // <---------------------
    defs::hNtWriteFileSsn = ssn;                    // <---------------------
    let __status = defs::hNtWriteFile(
        self.handle.to_owned(), 
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        &mut iosb,
        buffer.as_mut_ptr() as *const u8,
        buffer.len(),
        &mut self.offset as *mut c_longlong,
        0
    );
    Ok((__status, iosb.Information))
}
```
```
0xC0000005: STATUS_ACCESS_VIOLATION
The instruction at 0x%08lx referenced memory at 0x%08lx. The memory could not be %s.
```
This is why all value changes are placed in this way:
```rust
let func_name = hide!("NtWriteFile");
let (ssn, syscall_addr) = match self.__find_nt_function_ssn(func_name) {
    Some(_ssn) => _ssn,
    None => {
        return Err(format!("could not find ssn for {:?}", func_name));
    }
};
unsafe {
    defs::hNtWriteFileSsn = ssn;
    defs::hNtWriteFileSyscallAddr = syscall_addr;    
}
...
unsafe {
    let __status = defs::hNtWriteFile
    ...
}
```