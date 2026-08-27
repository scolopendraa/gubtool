mov rdx, rdi
mov rcx, rbx
push rax
push rdi
movabs rax, OFFSET disable_all_flag
cmp BYTE PTR [rax], 0x0
je check_individual
movabs rax, OFFSET target_loc
mov rax, QWORD PTR [rax]
test rax, rax
je skip_call
mov rax, QWORD PTR [rax+0xe8]
test rax, rax
je skip_call
mov rax, QWORD PTR [rax+0x18]
test rax, rax
je skip_call
mov rax, QWORD PTR [rax+0x20]
cmp rax, rcx
jne skip_call
check_individual:
movabs rax, OFFSET buffer_loc
mov edi, DWORD PTR [rax]
lea rax, [rax+4]
loop_start:
cmp edi, 0x0
jle normal
cmp QWORD PTR [rax], rcx
je skip_call
add rax, 0x8
dec edi
jmp loop_start
skip_call:
pop rdi
pop rax
jmp OFFSET hook_loc_skip_call
normal:
pop rdi
pop rax
jmp OFFSET hook_loc_normal
