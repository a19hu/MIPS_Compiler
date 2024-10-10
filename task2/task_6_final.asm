.data
    value1: .word 25
    value2: .word 7
    threshold: .word 30

.text
    lw $t0, value1
    lw $t1, value2
    lw $t4, threshold

    addi $t2, $t0, -5
    beq $t2, $t1, check_greater

    sub $t3, $t0, $t1
    beq $zero, $zero, end_if

check_greater:
    slt $t5, $t0, $t4
    beq $t5, $zero, greater_or_equal

    add $t3, $t0, $t1
    beq $zero, $zero, end_if

greater_or_equal:
    and $t3, $t0, $t1

end_if:
    
