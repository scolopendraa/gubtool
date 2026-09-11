push rax
movabs rax, OFFSET property_loc
movups xmm15, XMMWORD PTR [rax]
pop rax
movups XMMWORD PTR [rax+OFFSET property_offset], xmm15
xorps xmm15, xmm15
ret