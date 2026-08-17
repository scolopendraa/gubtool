push ebp
push edi
push esi
push ebx
mov ebp, esp
sub esp, 0x220
lea ecx, [ebp-0x110]
push OFFSET event_id
mov eax, OFFSET fn_event_ctor
call eax
lea ecx, [ebp-0x190]
lea edx, [ebp-0x1c0]
mov DWORD PTR [ecx+0x70], edx
lea eax, [ebp-0x1e0]
mov DWORD PTR [edx+0x4], eax
mov DWORD PTR [eax+0xc], OFFSET map_id
mov DWORD PTR [eax+0x10], 0x0
lea eax, [ebp-0x200]
mov DWORD PTR [edx+0x10], eax
xor eax, eax
mov edx, DWORD PTR [ecx+0x70]
mov ecx, DWORD PTR [edx+0x10]
mov DWORD PTR [ecx], eax
mov DWORD PTR [ecx+0x4], eax
mov DWORD PTR [ecx+0x8], eax
mov DWORD PTR [ecx+0xc], eax
mov ebx, OFFSET params_len
mov DWORD PTR [ebp-0x14], ebx
xor edi, edi
mov esi, OFFSET params_loc
param_loop:
mov eax, DWORD PTR [esi+edi*4]
lea ecx, [ebp-0x110]
mov edx, DWORD PTR [ecx]
mov edx, DWORD PTR [edx+0x4]
push 0x2
push eax
lea eax, [edi+0x1]
push eax
call edx
inc edi
cmp edi, ebx
jl param_loop
lea ecx, [ebp-0x190]
push 0x0
lea eax, [ebp-0x110]
push eax
mov eax, OFFSET fn_execute_event
call eax
cleanup:
mov esp, ebp
pop ebx
pop esi
pop edi
pop ebp
ret
