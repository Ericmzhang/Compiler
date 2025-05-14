.global main
main:
   movl    $4, %eax
   push    %eax
   movl    $2, %eax
   movl    %eax,   %ecx
   pop     %eax
   cdq
   idivl    %ecx
   ret
