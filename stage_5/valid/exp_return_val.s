.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl   $0, %eax
   push %eax
   movl   $0, %eax
   push %eax
   movl    $4, %eax
   movl   %eax, -8(%ebp)
   movl   %eax, -4(%ebp)
   movl   -4(%ebp), %eax
   push    %eax
   movl   -8(%ebp), %eax
   movl    %eax,   %ecx
   pop     %eax
   subl    %ecx,   %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
