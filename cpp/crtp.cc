#include <iostream>

// https://en.wikipedia.org/wiki/Curiously_recurring_template_pattern

template <typename>
class A
{
public:
    void foo()
    {
        std::cerr << "A::foo" << std::endl;
    }

    void bar()
    {
        std::cerr << "A::bar" << std::endl;
    }
};

class B : public A<B>
{
public:
    void foo()
    {
        std::cerr << "B::foo" << std::endl;
    }
};

int main()
{
    B b;
    b.foo();
    b.bar();
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
