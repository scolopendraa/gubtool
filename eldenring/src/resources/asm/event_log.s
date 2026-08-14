mov QWORD PTR [rsp+0x8], rbx
push rax
push rbx
push rdi
push r9
movabs r9, OFFSET write_index
mov edi, DWORD PTR [r9]
mov eax, edi
imul eax, eax, 0x5
movabs rbx, OFFSET buffer
add rbx, rax
mov DWORD PTR [rbx], edx
mov BYTE PTR [rbx+0x4], r8b
inc edi
and edi, 0x1FF
mov DWORD PTR [r9], edi
pop r9
pop rdi
pop rbx
pop rax
jmp OFFSET hook_loc
