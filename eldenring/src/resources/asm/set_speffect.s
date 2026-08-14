movabs rcx, OFFSET chr_ins_ptr
mov rdx, OFFSET speffect_id
movabs rax, OFFSET fn_set_speffect
sub rsp, 0x28
call rax
add rsp, 0x28
ret
