class C
{
public:
    static const int aS[];
    static const int* getS()
    {
        return aS;
    }
};

int foo(const int* pS)
{
    return 0;
}
#define FOO(a) foo(a)

int main()
{
    FOO(C::aS);
}
