movabs rcx, OFFSET path_loc
movabs rax, OFFSET load_library_w_loc
mov rax, [rax]
sub rsp, 0x28
call rax
add rsp, 0x28
ret