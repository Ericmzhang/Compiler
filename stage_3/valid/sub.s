.global main
main:
   movl    $1, %eax
   push    %eax
   movl    $2, %eax
   movl    %eax,   %ecx
   pop     %eax
   subl    %ecx,   %eax
   ret
