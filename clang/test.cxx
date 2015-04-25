#include <iostream>

class C
{
public:
    int nX;
    C()
        : nX(0)
    {
    }
};

int main(void)
{
    C aC;
    aC.nX = 1;
    std::cerr << aC.nX << std::endl;
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
