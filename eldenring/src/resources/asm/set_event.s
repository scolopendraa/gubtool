movabs rcx, OFFSET virt_mem_flag
mov rdx, OFFSET event_id
mov r8, OFFSET state
movabs rax, OFFSET fn_set_event
mov rcx, QWORD PTR [rcx]
sub rsp, 0x20
call rax
add rsp, 0x20
ret
