.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $1, %eax
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
   movl   %ebp, %esp
   pop    %ebp 
   ret
