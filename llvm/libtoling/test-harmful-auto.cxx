#include <map>
#include <memory>

template <typename T> T func() { return T(); }

int func2() { return 0; }

int main()
{
    // KO
    auto x = 42;

    // OK
    std::map<int, int> m;
    auto it = m.find(42);

    // OK
    auto pI = std::make_shared<int>();
    auto i = func<int>();

    // KO
    auto i2 = func2();

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
