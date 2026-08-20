movups xmm0, XMMWORD PTR [rip+OFFSET new_val]
movups XMMWORD PTR [rax+OFFSET property_offset], xmm0
jmp OFFSET hook_loc
