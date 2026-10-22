push eax
push ebx
push edi
push ecx
mov ecx, OFFSET write_index
mov edi, DWORD PTR [ecx]
mov eax, edi
imul eax, eax, 0x5
lea ebx, [buffer]
add ebx, eax
mov DWORD PTR [ebx], esi
mov al, BYTE PTR [ebp+0xC]
mov BYTE PTR [ebx+0x4], al
inc edi
and edi, 0x1FF
mov DWORD PTR [ecx], edi
pop ecx
pop edi
pop ebx
pop eax
mov eax, 0xD1B71759
jmp OFFSET hook_loc