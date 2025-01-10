#include <functional>

class C
{
  public:
    std::function<void()> f;
    C(unsigned id) : f([&]() { (void)id; }) {}
};

class D
{
  public:
    std::function<void()> f;
    D(unsigned id) : f([id]() { (void)id; }) {}
};

int main() { C c(0); }

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
