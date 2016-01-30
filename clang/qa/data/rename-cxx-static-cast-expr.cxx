class C
{
public:
    int getValue() const { return 0; }
};

int main()
{
    C* pC = 0;
    static_cast<const C&>(*pC).getValue();
    static_cast<const C*>(0)->getValue();
}
