#include <vector>
#include <algorithm>
#include <iostream>

// This mimics Python's filter() function.

int main()
{
    std::vector<int> v1;
    v1.push_back(1);
    v1.push_back(2);
    v1.push_back(3);
    std::vector<int> v2;

    std::copy_if(v1.begin(), v1.end(), std::back_inserter(v2), [](int i)
    {
        return i >= 2;
    });

    for (const int& i : v2)
    {
        std::cout << i << std::endl;
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
