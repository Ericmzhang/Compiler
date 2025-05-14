.global main
main:
   movl    $5, %eax
   cmpl   $0, %eax 
   neg     %eax 
   sete   %al

   ret
