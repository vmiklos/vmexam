#include <memory>

template <typename T> T func() { return T(); }

int func2() { return 0; }

int main()
{
    auto x = 42;

    // std::map<int, int> m;
    // auto it = m.find(42);

    auto pI = std::make_shared<int>();
    auto i = func<int>();

    auto i2 = func2();

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
