.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $5, %eax
   cmpl   $0, %eax 
   neg     %eax 
   sete   %al

   movl   %ebp, %esp
   pop    %ebp 
   ret
