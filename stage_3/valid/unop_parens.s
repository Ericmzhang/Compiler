.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $1, %eax
   push    %eax
   movl    $1, %eax
   pop     %ecx
   addl    %ecx,   %eax
   not     %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
