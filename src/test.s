.global main
main:
   movl    $12, %eax
   neg     %eax
   push    %eax
   movl    $5, %eax
   movl    %eax,   %ecx
   pop     %eax
   cdq
   idivl    %ecx
   ret
