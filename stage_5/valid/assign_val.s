.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl   $0, %eax
   push %eax
   movl    $0, %eax
   movl   %eax, -4(%ebp)
   push %eax
   movl   -8(%ebp), %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
