mov edx, OFFSET obj_id
mov ecx, OFFSET map_id
movabs rax, OFFSET fn_map_entity
sub rsp, 0x28
call rax
add rsp, 0x28
test rax, rax
jz save
mov rax, QWORD PTR [rax+0xb8]
test rax, rax
jz save
mov rax, QWORD PTR [rax+0x40]
test rax, rax
jz save
mov rax, QWORD PTR [rax+0x48]
save:
movabs rdx, OFFSET saved_state_act_ctrl
mov QWORD PTR [rdx], rax
ret
