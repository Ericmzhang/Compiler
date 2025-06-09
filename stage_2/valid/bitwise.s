.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $12, %eax
   not     %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
