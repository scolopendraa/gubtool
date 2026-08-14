movabs r14, OFFSET game_man_imp
mov r14, QWORD PTR [r14]
mov r13, r14
mov r14, QWORD PTR [r14+0xa8]
mov r14, QWORD PTR [r14+0x10]
mov r14, QWORD PTR [r14+0x10]
movabs r8, OFFSET adjust_quantity_flag
cmp BYTE PTR [r8], 0x1
jne skip_adjust
mov rcx, QWORD PTR [r14+0x10]
movabs r9, OFFSET item_id
movabs r8, OFFSET stack_count
movabs rdx, OFFSET current_quantity
mov r9d, DWORD PTR [r9]
movabs rax, OFFSET fn_current_item_quantity_check
sub rsp, 0x30
call rax
add rsp, 0x30
movabs r9, OFFSET quantity
movabs r10, OFFSET current_quantity
movabs r11, OFFSET max_quantity
movzx eax, WORD PTR [r9]
add eax, DWORD PTR [r10]
cmp eax, DWORD PTR [r11]
jle skip_adjust
mov eax, DWORD PTR [r11]
sub eax, DWORD PTR [r10]
mov WORD PTR [r9], ax
skip_adjust:
sub rsp, 0x208
mov rcx, r14
movabs r14, OFFSET item_count
movabs r15, OFFSET item_struct
mov rdx, r15
mov r8d, DWORD PTR [r14]
xor r9d, r9d
movabs rax, OFFSET fn_item_spawn
call rax
mov rdx, r15
mov r8d, DWORD PTR [r14]
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
ret
