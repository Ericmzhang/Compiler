.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $2, %eax
   push    %eax
   movl    $3, %eax
   push    %eax
   movl    $4, %eax
   pop     %ecx
   addl    %ecx,   %eax
   pop     %ecx
   imul    %ecx,   %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
