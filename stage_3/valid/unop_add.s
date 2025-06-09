.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $2, %eax
   not     %eax
   push    %eax
   movl    $3, %eax
   pop     %ecx
   addl    %ecx,   %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
