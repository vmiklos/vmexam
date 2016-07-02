#include <iostream>

// This intentionally does a stack-use-after-return error for asan demo
// purposes.
//
// Run it like this: 'ASAN_OPTIONS=detect_stack_use_after_return=1 ./asan'

class C
{
public:
    int* pointer = nullptr;
};

void foo(C& c)
{
    int x = 0;
    c.pointer = &x;
}

int main()
{
    C c;
    foo(c);
    std::cerr << *c.pointer << std::endl;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
