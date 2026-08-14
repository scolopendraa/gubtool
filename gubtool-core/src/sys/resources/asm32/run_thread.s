push 0x0 # lpThreadId
push 0x0 # dwCreationFlags
push 0x0 # lpParameter
push OFFSET code_address
push 0x0 # dwStackSize
push 0x0 # lpThreadAttributes
mov eax, ds:OFFSET create_thread
call eax
test eax, eax
je finish
push eax
mov eax, ds:OFFSET close_handle
call eax
test eax, eax
je finish
mov edi, OFFSET flag_loc
mov BYTE PTR [edi], 0x1
finish:
int3
