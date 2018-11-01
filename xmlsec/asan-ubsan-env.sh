# The C subset of LibreOffice's sanitizer config.
export ASAN_OPTIONS=strip_path_prefix=$(pwd)/:handle_ioctl=1:detect_leaks=1:allow_user_segv_handler=1:use_sigaltstack=0:detect_deadlocks=1:intercept_tls_get_addr=1:check_initialization_order=1:detect_stack_use_after_return=1:strict_init_order=1:detect_invalid_pointer_pairs=1
export LSAN_OPTIONS=print_suppressions=0:report_objects=1
export UBSAN_OPTIONS=print_stacktrace=1
export CC="clang -fno-sanitize-recover=undefined,integer -fsanitize=address -fsanitize=undefined -fsanitize=local-bounds"
bash
