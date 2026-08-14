sub rsp, 0x40
xor rcx, rcx # lpThreadAttributes
xor rdx, rdx # dwStackSize
xor r9, r9 # lpParameter
movabs r8, OFFSET code_address
mov QWORD PTR [rsp+0x20], 0x0 # dwCreationFlags
mov QWORD PTR [rsp+0x28], 0x0 # lpThreadId
movabs rax, OFFSET create_thread
call QWORD PTR [rax]
test rax, rax
je finish
mov rcx, rax
movabs rax, OFFSET close_handle
call QWORD PTR [rax]
test rax, rax
je finish
movabs rdi, OFFSET flag_loc
mov BYTE PTR [rdi], 0x1
finish:
add rsp, 0x40
int3