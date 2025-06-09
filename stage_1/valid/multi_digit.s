.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $100, %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
