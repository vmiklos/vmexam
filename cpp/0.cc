template<typename T>
struct Tmp
{
    void foo(T* p)
    {
        p->run();
    }
};

int main()
{
    Tmp<int> t;
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
