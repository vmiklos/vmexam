#include <any>
#include <iostream>
#include <string>

int main()
{
    std::any a;
    a = std::string("s");
    // a = 42;
    auto p = std::any_cast<std::string>(&a);
    if (p)
    {
        std::cerr << "debug, main: p is '" << *p << "'\n";
    }
    else
    {
        std::cerr << "debug, main: p is empty\n";
    }
}
