push rbx
push r13
push r14
push r15
movabs r14, OFFSET game_man_imp
mov r14, QWORD PTR [r14]
mov r13, r14
mov r14, QWORD PTR [r14+0xa8]
mov r14, QWORD PTR [r14+0x10]
mov r14, QWORD PTR [r14+0x10]
movabs rbx, OFFSET item_args
cmp BYTE PTR [rbx], 0x1 # adjust_quantity_flag
jne skip_adjust
mov rcx, QWORD PTR [r14+0x10]
lea rdx, [rbx+0x4] # current_quantity
lea r8, [rbx+0x10] # stack_count
mov r9d, DWORD PTR [rbx+0x18] # item_id
movabs rax, OFFSET fn_current_item_quantity_check
sub rsp, 0x30
call rax
add rsp, 0x30
movzx eax, WORD PTR [rbx+0x20] # quantity
add eax, DWORD PTR [rbx+0x4] # current_quantity
cmp eax, DWORD PTR [rbx+0x8] # max_quantity
jle skip_adjust
mov eax, DWORD PTR [rbx+0x8]
sub eax, DWORD PTR [rbx+0x4]
mov WORD PTR [rbx+0x20], ax
skip_adjust:
sub rsp, 0x208
mov rcx, r14
lea rdx, [rbx+0x14] # item_struct
mov r8d, DWORD PTR [rbx+0xc] # item_count
xor r9d, r9d
movabs rax, OFFSET fn_item_spawn
call rax
lea rdx, [rbx+0x14]
mov r8d, DWORD PTR [rbx+0xc]
movabs r15, OFFSET stack_loc
mov rcx, r15
mov r9d, 0x1
movabs rax, OFFSET fn_build_item_dialogue
call rax
mov rcx, QWORD PTR [r13+0x22e0]
mov rdx, r15
movabs rax, OFFSET fn_show_item_dialogue
call rax
add rsp, 0x208
pop r15
pop r14
pop r13
pop rbx
ret
