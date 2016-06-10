#include <vector>
#include <string>
#include <iostream>
#include <random>

// This mimics Python's random.choice() function.
const std::string& random_choice(const std::vector<std::string>& v)
{
    std::random_device random_device;
    std::mt19937 engine(random_device());
    std::uniform_int_distribution<int> dist(0, v.size() - 1);
    return v[dist(engine)];
}

int main()
{
    std::vector<std::string> v;
    v.push_back("a");
    v.push_back("b");
    v.push_back("c");

    std::cout << random_choice(v) << std::endl;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
