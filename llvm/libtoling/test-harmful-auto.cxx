#include <map>
#include <memory>

int main()
{
    auto x = 42;

    std::map<int, int> m;
    auto it = m.find(42);

    auto i = std::make_shared<int>();

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
