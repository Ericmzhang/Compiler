.global main
main:
   movl    $1, %eax
   push    %eax
   movl    $1, %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   setge   %al  
   ret
