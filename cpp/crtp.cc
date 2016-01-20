#include <iostream>

// https://en.wikipedia.org/wiki/Curiously_recurring_template_pattern

template <typename Derived>
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

    void work()
    {
        static_cast<Derived*>(this)->foo();
        static_cast<Derived*>(this)->bar();
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
    b.work();
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
