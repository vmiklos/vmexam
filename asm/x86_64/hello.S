.section .rodata
.scanf_string:
    .asciz    "%d"
.printf_string:
    .asciz    "You gave %d\n"

.section .text
.global    main
main:
    # int i;
    subq $8, %rsp # reserve memory for a 8-byte local variable

    # scanf("%d", &i);
    movq $.scanf_string, %rdi # load scanf_string to rdi (first argument)
    leaq 0(%rsp), %rsi # load *address of* rsp+0 to rsi (second argument)
    call scanf

    # printf("You gave %d\n", i);
    movq $.printf_string, %rdi # load printf_string to rdi (first param)
    movq 0(%rsp), %rsi # load rsp+0 to rsi (second argument)
    call printf

    # return 0;
    addq $8, %rsp
    ret
