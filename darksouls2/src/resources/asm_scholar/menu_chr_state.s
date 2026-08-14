movabs rcx, OFFSET dl_back_allocator
mov r8d, OFFSET state
movabs rax, OFFSET fn_menu_chr_state
mov edx, 0x11
sub rsp, 0x28
call rax
add rsp, 0x28
ret
