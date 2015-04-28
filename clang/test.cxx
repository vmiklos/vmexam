#include "test.hxx"

C::C()
    : nX(0),
      nY(0)
{
}

int main(void)
{
    C aC;
    aC.nX = 1;
    int y = aC.nX;
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
