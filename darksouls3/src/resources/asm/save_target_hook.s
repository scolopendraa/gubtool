push rbp
movabs rbp, OFFSET saved_pointer_loc
mov QWORD PTR [rbp], rax
pop rbp
mov rax, QWORD PTR [rax+OFFSET modules_offset]
jmp OFFSET hook_loc