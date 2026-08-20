push ebp
push ebx
push esi
push edi
mov ebx, DWORD PTR ds:OFFSET game_man_imp
mov edi, ebx
mov ebx, DWORD PTR [ebx+0x60]
mov ebx, DWORD PTR [ebx+0x8]
mov ebx, DWORD PTR [ebx+0x8]
mov ebp, OFFSET item_args
cmp BYTE PTR [ebp], 0x1
jne skip_adjust
mov ecx, DWORD PTR [ebx+0x8]
push DWORD PTR [ebp+0x18] # item_id
lea edx, [ebp+0x10] # stack_count
push edx
lea edx, [ebp+0x4] # current_quantity
push edx
mov eax, OFFSET fn_current_item_quantity_check
call eax
movzx eax, WORD PTR [ebp+0x20] # quantity
add eax, DWORD PTR [ebp+0x4] # current_quantity
cmp eax, DWORD PTR [ebp+0x8] # max_quantity
jle skip_adjust
mov eax, DWORD PTR [ebp+0x8]
sub eax, DWORD PTR [ebp+0x4]
mov WORD PTR [ebp+0x20], ax
skip_adjust:
mov ecx, ebx
push 0x0
push DWORD PTR [ebp+0xc] # item_count
lea esi, [ebp+0x14] # item_struct
push esi
mov eax, OFFSET fn_item_spawn
call eax
push 0x1
push DWORD PTR [ebp+0xc]
push esi
mov ebx, OFFSET stack_loc
push ebx
mov eax, OFFSET fn_build_item_dialogue
call eax
add esp, 0x10
mov ecx, DWORD PTR [edi+0xCC4]
push ebx
mov eax, OFFSET fn_show_item_dialogue
call eax
pop edi
pop esi
pop ebx
pop ebp
ret
