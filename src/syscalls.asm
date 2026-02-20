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

EXTERN hNtCreateFileSyscallAddr:QWORD
EXTERN hNtWriteFileSyscallAddr:QWORD
EXTERN hNtOpenFileSyscallAddr:QWORD
EXTERN hNtCloseFileSyscallAddr:QWORD
EXTERN hNtReadFileSyscallAddr:QWORD
EXTERN hNtDeleteFileSyscallAddr:QWORD
EXTERN hNtQueryInformationFileSyscallAddr:QWORD

hNtCreateFile PROC
    mov r10, rcx
    mov eax, hNtCreateFileSsn
    jmp hNtCreateFileSyscallAddr
    ret
hNtCreateFile ENDP

hNtWriteFile PROC
    mov r10, rcx
    mov eax, hNtWriteFileSsn


    jmp hNtWriteFileSyscallAddr
    ret
hNtWriteFile ENDP

hNtReadFile PROC
    mov r10, rcx
    mov eax, hNtReadFileSsn
    jmp hNtReadFileSyscallAddr
    ret
hNtReadFile ENDP

hNtOpenFile PROC
    mov r10, rcx
    mov eax, hNtOpenFileSsn
    jmp hNtOpenFileSyscallAddr
    ret
hNtOpenFile ENDP

hNtCloseFile PROC
    mov r10, rcx
    mov eax, hNtCloseFileSsn
    jmp hNtCloseFileSyscallAddr
    ret
hNtCloseFile ENDP

hNtDeleteFile PROC
    mov r10, rcx
    mov eax, hNtDeleteFileSsn
    jmp hNtDeleteFileSyscallAddr
    ret
hNtDeleteFile ENDP

hNtQueryInformationFile PROC
    mov r10, rcx
    mov eax, hNtQueryInformationFileSsn
    jmp  hNtQueryInformationFileSyscallAddr
    ret
hNtQueryInformationFile ENDP

END