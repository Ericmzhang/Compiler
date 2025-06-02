.global main
main:
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
   cmpl    $0,   %eax
   jne      _clause3
   jmp      _end2
_clause3:
   movl    $0, %eax
   cmpl $0, %eax
   movl $0, %eax
   setne %al 
_end2:
   ret
