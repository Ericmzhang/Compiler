.global main
main:
   movl    $0, %eax
   cmpl   $0, %eax 
   neg     %eax 
   sete   %al

   ret
