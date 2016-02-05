#include <functional>
#include <iostream>

/// Trivial C++11 equivalent of Boost.ScopeExit

namespace
{
bool gFoo = false;
}

class ScopeGuard
{
    std::function<void ()> m_aFunc;

public:
    template <typename FuncType>
    explicit ScopeGuard(const FuncType& rFunc)
        : m_aFunc(rFunc)
    {
    }

    ~ScopeGuard();
};

ScopeGuard::~ScopeGuard()
{
    m_aFunc();
}

int foo()
{
    gFoo = true;
    ScopeGuard g([&]()
    {
        gFoo = false;
    });
}

int main()
{
    foo();
    std::cerr << "gFoo is " << gFoo << std::endl;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
