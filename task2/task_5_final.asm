.data
    num1: .word 10
    num2: .word 20
    result: .word 0

.text
    lw $t0, num1
    lw $t1, num2
    add $t2, $t0, $t1
    sw $t2, result

