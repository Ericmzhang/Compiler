.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $12, %eax
   neg     %eax
   push    %eax
   movl    $5, %eax
   movl    %eax,   %ecx
   pop     %eax
   cdq
   idivl    %ecx
   movl   %ebp, %esp
   pop    %ebp 
   ret
