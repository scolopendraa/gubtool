mov rbp, rsp
sub rsp, 0x408
lea rcx, [rbp-0x1b0]
lea rdx, [rbp-0x1c0]
mov DWORD PTR [rdx], 0x0
mov WORD PTR [rdx+0x4], 0xFFFF
mov DWORD PTR [rdx+0x8], 0x0
lea r8, [rbp-0x1d0]
mov QWORD PTR [r8], 0x0
mov QWORD PTR [r8+0x8], 0x0
lea r9, [rbp-0x1e0]
mov QWORD PTR [r9], 0x0
mov DWORD PTR [rsp+0x28], 0x0
mov DWORD PTR [rsp+0x30], 0xFFFFFFFF
mov DWORD PTR [rsp+0x38], 0xFFFFFFFF
movabs rax, OFFSET fn_emk_event_ins_ctor
call rax
lea rcx, [rbp-0x210]
mov DWORD PTR [rcx], OFFSET group_id
mov DWORD PTR [rcx+0x4], OFFSET command_id
lea r8, [rbp-0x1b0]
mov QWORD PTR [r8+0xd0], rcx
movabs rax, OFFSET args_location
lea rcx, [rax]
mov QWORD PTR [r8+0xd8], rcx
movabs rax, OFFSET cs_emk_system_base
mov rax, [rax]
mov rcx, QWORD PTR [rax+0x28]
mov eax, 0x3ca3d70a
movd xmm1, eax
movabs rax, OFFSET fn_emevd_switch
call rax
lea rcx, [rbp-0x1b0]
mov rax, QWORD PTR [rcx]
call QWORD PTR [rax]
add rsp, 0x408
ret
