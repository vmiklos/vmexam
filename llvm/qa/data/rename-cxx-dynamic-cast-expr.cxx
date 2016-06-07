class C
{
public:
    int getValue() const
    {
        return 0;
    }
};

int main()
{
    C* pC = new C();
    dynamic_cast<const C&>(*pC).getValue();
    dynamic_cast<const C*>(pC)->getValue();
}
