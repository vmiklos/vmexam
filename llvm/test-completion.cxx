#include <iostream>

namespace ns
{

class C
{
public:
    /// Foo does x.
    int foo(int x);
    int foo(const std::string& rString);
    int bar(int x);
};

int C::foo(int x)
{
    int y = 0;
    return 0;
}
int C::foo(const std::string& rString)
{
    std::string aString(rString);
    return 0;
}
int C::bar(int x)
{
    int y = 0;
    return 0;
}

}

int main()
{
    std::cout << "hello" << std::endl;
    ns::C c;
    c.foo(0);
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
