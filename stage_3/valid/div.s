.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $4, %eax
   push    %eax
   movl    $2, %eax
   movl    %eax,   %ecx
   pop     %eax
   cdq
   idivl    %ecx
   movl   %ebp, %esp
   pop    %ebp 
   ret
