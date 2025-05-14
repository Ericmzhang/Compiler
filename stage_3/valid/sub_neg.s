.global main
main:
   movl    $2, %eax
   push    %eax
   movl    $1, %eax
   neg     %eax
   movl    %eax,   %ecx
   pop     %eax
   subl    %ecx,   %eax
   ret
