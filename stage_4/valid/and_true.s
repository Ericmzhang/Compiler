.global main
main:
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
   ret
