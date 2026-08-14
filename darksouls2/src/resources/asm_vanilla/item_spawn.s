mov ebx, DWORD PTR ds:OFFSET game_man_imp
mov edi, ebx
mov ebx, DWORD PTR [ebx+0x60]
mov ebx, DWORD PTR [ebx+0x8]
mov ebx, DWORD PTR [ebx+0x8]
cmp BYTE PTR ds:OFFSET adjust_quantity_flag, 0x1
jne skip_adjust
mov ecx, DWORD PTR [ebx+0x8]
mov edx, DWORD PTR ds:OFFSET item_id
push edx
lea edx, ds:OFFSET stack_count
push edx
lea edx, ds:OFFSET current_quantity
push edx
mov eax, OFFSET fn_current_item_quantity_check
call eax
mov esi, OFFSET quantity
mov edx, OFFSET current_quantity
mov ecx, OFFSET max_quantity
movzx eax, WORD PTR [esi]
add eax, DWORD PTR [edx]
cmp eax, DWORD PTR [ecx]
jle skip_adjust
mov eax, DWORD PTR [ecx]
sub eax, DWORD PTR [edx]
mov WORD PTR [esi], ax
skip_adjust:
mov ecx, ebx
mov ebx, DWORD PTR ds:OFFSET item_count
mov esi, OFFSET item_struct
push 0x0
push ebx
push esi
mov eax, OFFSET fn_item_spawn
call eax
push 0x1
push ebx
push esi
lea ebx, ds:OFFSET stack_loc
push ebx
mov eax, OFFSET fn_build_item_dialogue
call eax
add esp, 0x10
mov ecx, DWORD PTR [edi+0xCC4]
push ebx
mov eax, OFFSET fn_show_item_dialogue
call eax
ret
