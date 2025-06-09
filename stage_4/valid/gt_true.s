.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $1, %eax
   push    %eax
   movl    $0, %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   setg   %al  
   movl   %ebp, %esp
   pop    %ebp 
   ret
