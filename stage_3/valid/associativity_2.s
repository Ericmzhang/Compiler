.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $6, %eax
   push    %eax
   movl    $3, %eax
   movl    %eax,   %ecx
   pop     %eax
   cdq
   idivl    %ecx
   push    %eax
   movl    $2, %eax
   movl    %eax,   %ecx
   pop     %eax
   cdq
   idivl    %ecx
   movl   %ebp, %esp
   pop    %ebp 
   ret
