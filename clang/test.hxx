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

namespace ns
{
class C
{
public:
    int nX;
    int mnY;
    int m_nZ;

    C()
    {
    }
};
}

#define DELETEZ( p )    ( delete p,p = 0 )

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
