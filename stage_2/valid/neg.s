.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $5, %eax
   neg     %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
