.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $0, %eax
   push    %eax
   movl    $0, %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   setne   %al  
   movl   %ebp, %esp
   pop    %ebp 
   ret
