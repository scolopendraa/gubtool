push rbx
sub rsp, 0x28
movabs rbx, OFFSET game_manager_imp
mov rbx, [rbx]
mov rbx, [rbx+0x40]
lea rbx, [rbx+0x170]
mov esi, 0x2a
area_ctrl_loop:
mov rcx, QWORD PTR [rbx]
test rcx, rcx
je next_area
jmp lookup_enemy
next_area:
add rbx, 0x8
dec rsi
jne area_ctrl_loop
jmp exit
lookup_enemy:
push rdi
push rbx
push rsi
sub rsp, 0x20
mov rdi, rcx
xor ebx, ebx
enemy_generator_ctrl_loop:
mov rax, QWORD PTR [rdi+0x20]
mov rcx, QWORD PTR [rax+rbx*0x8]
test rcx, rcx
je wrong_enemy
movzx eax, WORD PTR [rcx+0x88]
mov esi, OFFSET obj_id
cmp eax, esi
jne wrong_enemy
mov edx, OFFSET bonfire_id
movabs rax, OFFSET fn_reset_enemy
call rax
wrong_enemy:
inc ebx
cmp ebx, DWORD PTR [rdi+0x28]
jc enemy_generator_ctrl_loop
cleanup:
add rsp, 0x20
pop rsi
pop rbx
pop rdi
jmp next_area
exit:
add rsp, 0x28
pop rbx
ret
