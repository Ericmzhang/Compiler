.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl   %ebp, %esp
   pop    %ebp 
   movl   $0, %eax
   ret
