#include <map>
#include <memory>

template <typename T> T func() { return T(); }

struct S
{
    template <typename T> static T staticFunc() { return T(); }
};

int func2() { return 0; }

enum E
{
    E1,
    E2
};

int main()
{
    // KO
    auto x = 42;
    auto i2 = func2();

    // OK
    std::map<int, int> m;
    auto it = m.find(42);
    auto pI = std::make_shared<int>();
    auto i = func<int>();
    auto si = S::staticFunc<int>();
    auto l = static_cast<long>(42);
    auto e = static_cast<E>(0);

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
