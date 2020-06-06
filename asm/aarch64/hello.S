.section .rodata
.scanf_string:
    .asciz    "%d"
.printf_string:
    .asciz    "You gave %d\n"

.section .text
.global main
main:
    # int i;
    # store x29 (frame pointer) and x30 (link register) on stack
    stp x29, x30, [sp, -32]!

    # scanf("%d", &i);
    # load scanf_string to x0 (first argument)
    adrp x0, .scanf_string
    add x0, x0, :lo12:.scanf_string
    # load *address of* sp+28 to x1 (second argument)
    add x1, sp, 28
    bl scanf

    # printf("You gave %d\n", i);
    # load printf_string to x0 (first param)
    adrp x0, .printf_string
    add x0, x0, :lo12:.printf_string
    # load sp+28 to x1 (second argument)
    ldr x1, [sp, 28]
    bl printf

    # return;
    ldp x29, x30, [sp], 32
    ret
