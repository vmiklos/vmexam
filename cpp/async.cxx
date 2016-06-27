#include <future>
#include <vector>
#include <unistd.h>
#include <iostream>

void f()
{
    std::cerr << "f start" << std::endl;
    sleep(1);
    std::cerr << "f end" << std::endl;
}

int main()
{
    // Shows that libstdc++ means "std::launch::deferred" by
    // "std::launch::async | std::launch::deferred" currently (g++ 4.8).
    std::cerr << "main start" << std::endl;
    std::vector<std::future<void>> futures;
    for (int i = 0; i < 10; ++i)
        futures.push_back(std::async(f));
    std::cerr << "main after std::async() calls, before waits" << std::endl;
    for (int i = 0; i < 10; ++i)
        futures[i].wait();
    std::cerr << "main end" << std::endl;
}
