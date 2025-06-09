.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $2, %eax
   push %eax
   movl    $0, %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
