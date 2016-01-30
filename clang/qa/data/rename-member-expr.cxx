class C
{
public:
    int nX;
};

int foo(int x)
{
    return 0;
}
#define FOO(a) foo(a)

int main()
{
    C aC;
    aC.nX = 1;
    FOO(aC.nX);
    int y = aC.nX;
}
