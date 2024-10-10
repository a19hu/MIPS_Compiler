.data
    a: .word 15
    b: .word 10
    c: .word 0

.text
    lw $t0, a
    lw $t1, b
    slt $t2, $t1, $t0
    beq $t2, $zero, else_case
    sub $t3, $t0, $t1
    j end
else_case:
    sub $t3, $t1, $t0
end:
    sw $t3, c

