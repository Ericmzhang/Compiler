.global main
main:
   movl    $2, %eax
   push    %eax
   movl    $2, %eax
   push    %eax
   movl    $0, %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   setg   %al  
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   sete   %al  
   ret
