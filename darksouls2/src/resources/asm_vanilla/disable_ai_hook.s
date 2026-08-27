mov eax, DWORD PTR [esi]
mov edx, DWORD PTR [eax+0x1c]
push eax
push edi
cmp BYTE PTR ds:OFFSET disable_all_flag, 0x0
je check_individual
mov eax, ds:OFFSET target_loc
test eax, eax
je skip_call
mov eax, DWORD PTR [eax+0xac]
test eax, eax
je skip_call
mov eax, DWORD PTR [eax+0xc]
test eax, eax
je skip_call
mov eax, DWORD PTR [eax+0x10]
cmp eax, esi
jne skip_call
check_individual:
mov eax, OFFSET buffer_loc
mov edi, DWORD PTR [eax]
lea eax, [eax+4]
loop_start:
cmp edi, 0x0
jle normal
cmp DWORD PTR [eax], esi
je skip_call
add eax, 0x8
dec edi
jmp loop_start
skip_call:
pop edi
pop eax
jmp OFFSET hook_loc_skip_call
normal:
pop edi
pop eax
jmp OFFSET hook_loc_normal
