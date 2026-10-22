mov ecx, OFFSET warp_manager
lea eax, [request_loc]
push eax
mov eax, OFFSET fn_request_warp
call eax
ret