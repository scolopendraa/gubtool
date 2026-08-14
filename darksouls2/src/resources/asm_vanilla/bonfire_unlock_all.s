push esi
push edi
push ebp
push ebx
mov ebx, OFFSET bonfire_manager
mov esi, OFFSET fn_bonfire_unlock
mov edi, DWORD PTR [ebx+0x14]
test edi, edi
je done
mov ebp, DWORD PTR [ebx+0x10]
push 0x0
loop_start:
mov eax, DWORD PTR [esp]
shl eax, 0x4
add eax, ebp
movzx eax, WORD PTR [eax]
push 0x1
push eax
mov ecx, ebx
call esi
inc DWORD PTR [esp]
mov eax, DWORD PTR [esp]
cmp eax, edi
jl loop_start
add esp, 0x4
done:
pop ebx
pop ebp
pop edi
pop esi
ret
