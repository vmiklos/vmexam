#include <random>
#include <string>
#include <vector>

const std::string& random_choice(const std::vector<std::string>& v)
{
    std::random_device random_device;
    std::mt19937 engine(random_device());
    std::uniform_int_distribution<int> dist(0, v.size() - 1);
    return v[dist(engine)];
}

extern "C"
{
    const char* myfunc(int x)
    {
        static std::string ret;
        std::vector<std::string> v{"a", "b", "c"};
        ret = "foo<b>bar</b>baz: " + random_choice(v);
        return ret.c_str();
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
