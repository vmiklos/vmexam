#include <iostream>
#include <iomanip>

int main()
{
    unsigned char c = 255;
    // C++ way to say printf("%02x...", ...);
    std::cerr << std::setfill('0') << std::setw(2) << std::hex << static_cast<int>(c) << std::endl;
    return 0;
}
