.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $1, %eax
   push    %eax
   movl    $2, %eax
   pop     %ecx
   addl    %ecx,   %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
