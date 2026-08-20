mov r14, QWORD PTR [rsp+0x70]
push rax
push rcx
push rdx
push r15
movabs rax, OFFSET world_area_time_impl
mov rax, [rax]
movabs rdx, OFFSET game_man
mov r15b, OFFSET stored_time_off
movzx r15, r15b
mov rdx, [rdx]
mov rcx, QWORD PTR [rax]
mov QWORD PTR [rdx+r15],rcx
mov rcx,QWORD PTR [rax+0x8]
mov QWORD PTR [rdx+r15+0x8],rcx
pop r15
pop rdx
pop rcx
pop rax
jmp OFFSET hook_loc
