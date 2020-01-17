template <typename T> class TD;
int main()
{
    // float or double?
    auto x = 1.0;
    // gives:
    // error: aggregate ‘TD<double> xType’ has incomplete type and cannot be
    // defined
    TD<decltype(x)> xType;
}
