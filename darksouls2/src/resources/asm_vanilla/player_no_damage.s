push eax
mov eax, [game_man_imp]
mov eax, DWORD PTR [eax+0x74]
cmp eax, esi
jne exit
mov ecx, DWORD PTR [esi+0xfc]
exit:
pop eax
mov DWORD PTR [esi+0xfc], ecx
jmp OFFSET hook_loc
