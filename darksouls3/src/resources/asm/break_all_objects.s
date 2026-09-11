movabs rdi, OFFSET world_obj_man_impl
mov rdi, QWORD PTR [rdi]
start:
movsxd rcx, ebx
imul rcx, rcx, 0x40288
add rcx, QWORD PTR [rdi+0x28]
sub rsp, 0x28
movabs rax, OFFSET fn_break_all_objects
call rax
add rsp, 0x28
inc ebx
cmp ebx, DWORD PTR [rdi+0x20]
jl start
ret
