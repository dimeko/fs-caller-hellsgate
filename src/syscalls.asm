.code

PUBLIC hNtCreateFile
PUBLIC hNtWriteFile
PUBLIC hNtReadFile
PUBLIC hNtOpenFile
PUBLIC hNtCloseFile
PUBLIC hNtDeleteFile
PUBLIC hNtQueryInformationFile

EXTERN hNtCreateFileSsn:DWORD
EXTERN hNtWriteFileSsn:DWORD
EXTERN hNtOpenFileSsn:DWORD
EXTERN hNtCloseFileSsn:DWORD
EXTERN hNtReadFileSsn:DWORD
EXTERN hNtDeleteFileSsn:DWORD
EXTERN hNtQueryInformationFileSsn:DWORD

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

hNtReadFile PROC
    mov r10, rcx
    mov eax, hNtReadFileSsn
    syscall
    ret
hNtReadFile ENDP

hNtOpenFile PROC
    mov r10, rcx
    mov eax, hNtOpenFileSsn
    syscall
    ret
hNtOpenFile ENDP

hNtCloseFile PROC
    mov r10, rcx
    mov eax, hNtCloseFileSsn
    syscall
    ret
hNtCloseFile ENDP

hNtDeleteFile PROC
    mov r10, rcx
    mov eax, hNtDeleteFileSsn
    syscall
    ret
hNtDeleteFile ENDP

hNtQueryInformationFile PROC
    mov r10, rcx
    mov eax, hNtQueryInformationFileSsn
    syscall
    ret
hNtQueryInformationFile ENDP

END