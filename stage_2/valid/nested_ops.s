.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $3, %eax
   neg     %eax
   cmpl   $0, %eax 
   neg     %eax 
   sete   %al

   movl   %ebp, %esp
   pop    %ebp 
   ret
