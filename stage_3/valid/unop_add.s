.global main
main:
   movl    $2, %eax
   not     %eax
   push    %eax
   movl    $3, %eax
   pop     %ecx
   addl    %ecx,   %eax
   ret
