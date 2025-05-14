.global main
main:
   movl    $2, %eax
   push    %eax
   movl    $3, %eax
   pop     %ecx
   imul    %ecx,   %eax
   ret
