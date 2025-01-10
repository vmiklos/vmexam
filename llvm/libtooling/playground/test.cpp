void f(int first, int second)
{
    int third = 0;
    auto l = [&first, second, &third]() {};
}

int main() { f(1, 2); }

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
