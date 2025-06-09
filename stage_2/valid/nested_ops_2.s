.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $0, %eax
   not     %eax
   neg     %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
