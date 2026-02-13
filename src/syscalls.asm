.code

PUBLIC hNtCreateFile
PUBLIC hNtWriteFile

EXTERN hNtCreateFileSsn:DWORD
EXTERN hNtWriteFileSsn:DWORD

hNtCreateFile PROC
    mov r10, rcx
    mov eax, hNtCreateFileSsn
    syscall
    ret
hNtCreateFile ENDP

hNtWriteFile PROC
    mov r10, rcx
    mov eax, hNtWriteFileSsn
    syscall
    ret
hNtWriteFile ENDP

END
