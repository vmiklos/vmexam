class C
{
public:
    int getValue()
    {
        return 0;
    }
};

int main()
{
    const C* pC = new C();
    const_cast<C*>(pC)->getValue();
}
