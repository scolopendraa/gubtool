push rbx
push r14
push r15
sub rsp, 0x40
movabs rbx, OFFSET item_args
movabs r15, OFFSET game_data_man
mov r15, QWORD PTR [r15]
mov r15, QWORD PTR [r15+0x10]
cmp BYTE PTR [rbx], 0x1
jne skip_qty_adjust
lea rcx, [r15+0x3d0]
lea rdx, [rbx+0xc]
movabs r14, OFFSET fn_get_item_quantity
call r14
mov edi, eax
mov rcx, QWORD PTR [r15+0x7b0]
lea rdx, [rbx+0xc]
call r14
add edi, eax
mov r8d, DWORD PTR [rbx+0x4]
sub r8d, edi
jle cleanup
mov r9d, DWORD PTR [rbx+0x10]
cmp r9d, r8d
jle skip_qty_adjust
mov DWORD PTR [rbx+0x10], r8d
skip_qty_adjust:
movabs rcx, OFFSET map_item_man
mov rcx, QWORD PTR [rcx]
lea rdx, [rbx+0x8]
movabs r8, OFFSET output
movabs rax, OFFSET fn_item_spawn
call rax
cleanup:
add rsp, 0x40
pop r15
pop r14
pop rbx
ret