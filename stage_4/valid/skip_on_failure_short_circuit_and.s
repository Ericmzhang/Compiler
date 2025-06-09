.global main
main:
   push   %ebp
   movl   %esp, %ebp
   movl    $0, %eax
   push %eax
   movl    $0, %eax
   push %eax
   movl   -4(%ebp), %eax
   cmpl    $0,   %eax
   jne      _clause2
   jmp      _end1
_clause2:
   movl    $5, %eax
   movl   %eax, -8(%ebp)
   cmpl $0, %eax
   movl $0, %eax
   setne %al 
_end1:
   movl   -8(%ebp), %eax
   movl   %ebp, %esp
   pop    %ebp 
   ret
