.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $2, %eax
   push    %eax
   movl    $1, %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   setl   %al  
   movl   %ebp, %esp
   pop    %ebp 
   ret
