#include <iostream>
#include <sstream>

int main()
{
	std::stringstream ss;
	ss << "foo" + 1;
	std::cerr << ss.str() << std::endl;
}
