.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $1, %eax
   cmpl    $0,   %eax
   jne      _clause2
   jmp      _end1
_clause2:
   movl    $1, %eax
   neg     %eax
   cmpl $0, %eax
   movl $0, %eax
   setne %al 
_end1:
   movl   %ebp, %esp
   pop    %ebp 
   ret
