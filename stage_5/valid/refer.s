.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $2, %eax
   push %eax
   movl   -4(%ebp), %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
