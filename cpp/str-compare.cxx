#include <cstring>
#include <iostream>
#include <string>

/// String compare without an std::string alloc or strlen().
bool equals(const std::string& a, const char* b, size_t n)
{
    if (a.length() != n)
    {
        return false;
    }

    return strncmp(a.c_str(), b, n) == 0;
}

template <size_t N> bool equals(const std::string& a, const char (&b)[N])
{
    return equals(a, b, N - 1);
}

int main()
{
    std::string s("a");
    std::cerr << "equals('a', 'a') is " << equals(s, "a") << std::endl;
    std::cerr << "equals('a', 'b') is " << equals(s, "b") << std::endl;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
