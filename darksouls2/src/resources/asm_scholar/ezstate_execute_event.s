mov rbp, rsp
sub rsp, 0x478
lea rcx, [rbp-0x3f8]
mov edx, OFFSET event_id
movabs rax, OFFSET fn_event_ctor
call rax
lea rcx, [rbp-0x1e8]
lea rdx, [rbp-0x128]
lea r8, [rbp-0x118]
mov QWORD PTR [rdx+0x8], r8
mov DWORD PTR [r8+0x18], OFFSET map_id
mov DWORD PTR [r8+0x1c], 0x0
xor eax, eax
lea r8, [rbp-0xe8]
mov QWORD PTR [r8], rax
mov QWORD PTR [r8+0x8], rax
mov QWORD PTR [r8+0x10], rax
mov QWORD PTR [r8+0x18], rax
mov QWORD PTR [rdx+0x20], r8
mov QWORD PTR [rcx+0xb0], rdx
lea r8, [rbp-0x418]
mov ebx, OFFSET params_len
mov DWORD PTR [rbp-0x1f8], ebx
xor edi, edi
movabs rax, OFFSET params_loc
lea rsi, [rax]
param_loop:
mov eax, DWORD PTR [rsi+rdi*4]
lea r8, [rbp-0xf8]
mov DWORD PTR [r8], eax
mov DWORD PTR [r8+0x8], 0x2
lea edx, [rdi+0x1]
lea rcx, [rbp-0x3f8]
mov rax, QWORD PTR [rcx]
call QWORD PTR [rax+0x8]
inc edi
cmp edi, ebx
jl param_loop
lea rcx, [rbp-0x1e8]
lea rdx, [rbp-0x3f8]
xor r8d, r8d
movabs rax, OFFSET fn_execute_event
call rax
add rsp, 0x478
ret
