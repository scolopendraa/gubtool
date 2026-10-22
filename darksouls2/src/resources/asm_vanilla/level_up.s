mov esi, DWORD PTR [current_level]
inc esi
mov edi, 0x0
cmp BYTE PTR [negative_flag], 0x1
je skip_lookup
mov ebx, OFFSET fn_level_lookup
loop_start:
push esi
call ebx
add esp, 0x4
add edi, eax
inc esi
cmp esi, DWORD PTR [new_level]
jle loop_start
skip_lookup:
mov DWORD PTR [required_souls], edi
mov ecx, DWORD PTR [current_souls]
cmp ecx, edi
jge enough_souls
sub edi, ecx
push edi
mov ecx, OFFSET stats_entity
mov eax, OFFSET fn_give_souls
call eax
mov ecx, OFFSET stats_entity
mov ecx, DWORD PTR [ecx+0xe8]
mov DWORD PTR [current_souls], ecx
mov edx, DWORD PTR [required_souls]
enough_souls:
sub ecx, edx
mov DWORD PTR [souls_after], ecx
mov ecx, OFFSET stats_entity
sub esp, 0xe0
lea eax, [buffer]
push eax
mov eax, OFFSET fn_level_up
call eax
add esp, 0xe0
ret