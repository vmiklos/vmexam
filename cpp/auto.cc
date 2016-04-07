#include <iostream>
#include <vector>

class C
{
public:
	C()
	{
		std::cerr << "C::C()" << std::endl;
	}

	C(const C&)
	{
		std::cerr << "C::C(const C&)" << std::endl;
	}
};

int main()
{
	std::vector<C> v;
	v.push_back(C());

	std::cerr << "before for, no copy will happen" << std::endl;
	for (auto& c : v)
	{
	}
	std::cerr << "after for" << std::endl;
	std::cerr << "before for, copy will happen" << std::endl;
	for (auto c : v)
	{
	}
	std::cerr << "after for" << std::endl;
}
