push OFFSET obj_id
push OFFSET map_id
mov eax, OFFSET fn_map_entity
call eax
test eax, eax
jz save
mov eax, DWORD PTR [eax+0x84]
test eax, eax
jz save
mov eax, DWORD PTR [eax+0x20]
test eax, eax
jz save
mov eax, DWORD PTR [eax+0x24]
save:
mov edx, OFFSET saved_state_act_ctrl
mov DWORD PTR [edx], eax
add esp, 0x8
ret
