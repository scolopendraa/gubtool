push ebx
push edi
mov edi, OFFSET game_manager_imp
mov edi, [edi]
mov edi, [edi+0x2c]
lea edi, [edi+0xc4]
mov ebx, 0x2a
area_ctrl_loop:
mov ecx, DWORD PTR [edi]
test ecx, ecx
je next_area
jmp lookup_enemy
next_area:
add edi, 0x4
dec ebx
jne area_ctrl_loop
jmp exit
lookup_enemy:
push edi
push esi
mov edi, ecx
xor esi, esi
enemy_generator_ctrl_loop:
mov eax, DWORD PTR [edi+0x10]
mov ecx, DWORD PTR [eax+esi*0x4]
test ecx, ecx
je wrong_enemy
movzx eax, WORD PTR [ecx+0x5c]
mov edx, OFFSET obj_id
cmp eax, edx
jne wrong_enemy
push DWORD PTR OFFSET bonfire_id
mov eax, OFFSET fn_reset_enemy
call eax
wrong_enemy:
inc esi
cmp esi, DWORD PTR [edi+0x14]
jc enemy_generator_ctrl_loop
cleanup:
pop esi
pop edi
jmp next_area
exit:
pop edi
pop ebx
ret
