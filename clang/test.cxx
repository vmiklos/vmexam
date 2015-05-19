#include "test.hxx"

const int C::aS[] = {
    0
};

C::C()
    : nX(0),
      nY(0),
      nZ(0),
      pX(0)
{
}

C::~C()
{
    DELETEZ( pX );
}

int foo(int x)
{
    return x;
}

#define FOO(a) foo(a)

int main(void)
{
    C aC;
    aC.nX = 1;
    int y = aC.nX;
    FOO(aC.nX);
    OSL_ENSURE(aC.nX, "test");
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
