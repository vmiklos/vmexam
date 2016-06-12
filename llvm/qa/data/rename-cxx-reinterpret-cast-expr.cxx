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
    void* pC = new C();
    reinterpret_cast<const C*>(pC)->getValue();
}
