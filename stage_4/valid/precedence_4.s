.global main
main:
   movl    $2, %eax
   push    %eax
   movl    $2, %eax
   pop    %ecx
   cmpl   %eax, %ecx
   movl   $0, %eax
   sete   %al  
   cmpl    $0,   %eax
   je      _clause2
   movl    $1,   %eax
   jmp      _end1
_clause2:
   movl    $0, %eax
   cmpl $0, %eax
   movl $0, %eax
   setne %al 
_end1:
   ret
