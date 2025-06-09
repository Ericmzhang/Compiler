.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $1, %eax
   push    %eax
   movl    $1, %eax
   neg     %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   setle   %al  
   movl   %ebp, %esp
   pop    %ebp 
   ret
