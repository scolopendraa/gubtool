push rax
movabs rax, OFFSET world_chr_man
mov rax, QWORD PTR [rax]
mov rax, QWORD PTR [rax+OFFSET player_ins_off]
cmp rax, QWORD PTR [rbp+0x8]
pop rax
je OFFSET skip_grab_jmp_location
mov edx, DWORD PTR [r14+0x44]
lea rcx, [rsp+0x40]
jmp OFFSET hook_loc
