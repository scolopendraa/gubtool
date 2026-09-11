mov rbp, rsp
sub rsp, 0x598
lea rcx, [rbp-0x420]
mov edx, OFFSET command_id
movabs rax, OFFSET fn_event_ctor
call rax
lea rcx, [rbp-0x460]
lea rdx, [rbp-0x520]
mov QWORD PTR [rcx+0x10], rdx
mov eax, 0x0
mov DWORD PTR [rcx+0x18], eax
mov DWORD PTR [rdx+0x4], eax
lea r8, [rbp-0x550]
mov QWORD PTR [rdx+0xa0], r8
mov QWORD PTR [r8+0x20], rdx
mov ebx, OFFSET params_len
xor edi, edi
movabs rsi, OFFSET params_loc
param_loop:
mov eax, DWORD PTR [rsi+rdi*4]
lea r8, [rbp-0x570]
mov DWORD PTR [r8], eax
mov DWORD PTR [r8+0x8], 0x2
lea edx, [rdi+0x1]
lea rcx, [rbp-0x420]
mov rax, QWORD PTR [rcx]
call QWORD PTR [rax+0x8]
inc edi
cmp edi, ebx
jl param_loop
lea rcx, [rbp-0x460]
lea rdx, [rbp-0x420]
movabs rax, OFFSET fn_execute_event
call rax
add rsp, 0x598
ret