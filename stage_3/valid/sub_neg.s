.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $2, %eax
   push    %eax
   movl    $1, %eax
   neg     %eax
   movl    %eax,   %ecx
   pop     %eax
   subl    %ecx,   %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
