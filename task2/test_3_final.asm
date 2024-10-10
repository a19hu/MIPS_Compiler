.data
    num1: .word 4
    num2: .word 5
    result: .word 0

.text
    lw $t0, num1
    lw $t1, num2
    addi $t2, $zero, 0
    addi $t3, $zero, 0

loop:
    beq $t3, $t1, end
    add $t2, $t2, $t0
    addi $t3, $t3, 1
    j loop

end:
    sw $t2, result
