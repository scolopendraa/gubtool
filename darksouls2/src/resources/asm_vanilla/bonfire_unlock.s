mov ecx, OFFSET bonfire_manager
mov edx, OFFSET bonfire_id
mov eax, OFFSET fn_bonfire_unlock
push 0x1 # show popup
push edx
call eax
ret
