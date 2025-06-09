.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $1, %eax
   neg     %eax
   push    %eax
   movl    $2, %eax
   neg     %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   setne   %al  
   movl   %ebp, %esp
   pop    %ebp 
   ret
