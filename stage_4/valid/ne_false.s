.global main
main:
   movl    $0, %eax
   push    %eax
   movl    $0, %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   setne   %al  
   ret
