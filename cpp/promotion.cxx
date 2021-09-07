#include <iostream>

int main()
{
    // demo for
    // https://stackoverflow.com/questions/46073295/implicit-type-promotion-rules
    {
        uint16_t a = 100;
        uint16_t b = 200;
        // implicitly promoted to (signed) int
        int64_t x = a - b;
        std::cerr << "x is a small negative number: " << x << std::endl;
    }
    {
        uint32_t a = 100;
        uint32_t b = 200;
        // already an (unsigned) int, so stays as-is
        int64_t x = a - b;
        std::cerr << "x is a large positive number: " << x << std::endl;
    }
}
