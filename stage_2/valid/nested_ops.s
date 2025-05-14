.global main
main:
   movl    $3, %eax
   neg     %eax
   cmpl   $0, %eax 
   neg     %eax 
   sete   %al

   ret
