.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $1, %eax
   push    %eax
   movl    $2, %eax
   movl    %eax,   %ecx
   pop     %eax
   subl    %ecx,   %eax
   push    %eax
   movl    $3, %eax
   movl    %eax,   %ecx
   pop     %eax
   subl    %ecx,   %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
