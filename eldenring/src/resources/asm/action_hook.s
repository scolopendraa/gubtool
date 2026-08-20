push r11
cmp edx, 0x11
je check_roll
cmp edx, 0x6
je check_jump
cmp edx, 0x10
je check_backstep
normal:
or QWORD PTR [r9+0x10], rax
return:
pop r11
ret
check_roll:
movabs r11, OFFSET roll_flag
cmp BYTE PTR [r11], 0x1
jne normal
jmp return
check_jump:
movabs r11, OFFSET jump_flag
cmp BYTE PTR [r11], 0x1
jne normal
jmp return
check_backstep:
movabs r11, OFFSET backstep_flag
cmp BYTE PTR [r11], 0x1
jne normal
jmp return
