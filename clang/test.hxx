class C
{
public:
    int nX;
    int nY;
    int nZ;
    int* pX;
    C();
    ~C();
};

#define DELETEZ( p )    ( delete p,p = 0 )

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
