.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $0, %eax
   push %eax
   movl   -4(%ebp), %eax
   cmpl    $0,   %eax
   je      _clause2
   movl    $1,   %eax
   jmp      _end1
_clause2:
   movl    $3, %eax
   movl   %eax, -4(%ebp)
   cmpl $0, %eax
   movl $0, %eax
   setne %al 
_end1:
   cmpl    $0,   %eax
   je      _clause3
   movl    $1,   %eax
   jmp      _end2
_clause3:
   movl    $4, %eax
   movl   %eax, -4(%ebp)
   cmpl $0, %eax
   movl $0, %eax
   setne %al 
_end2:
   movl   -4(%ebp), %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
