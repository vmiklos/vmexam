#include <algorithm>
#include <iostream>
#include <vector>

int main()
{
	std::vector<int> v = {0, 1, 2, 3};
	int cur = 4;
	int offset = 2;

#if 0
	// Take out 1
	int t = std::move(v[cur - 1 - offset]);
	// Move 2 and 3 to the left.
        std::move(v.data() + cur - offset, v.data() + cur,
                  v.data() + cur - 1 - offset);
	// Add back 1.
        v[cur - 1] = std::move(t);
#endif
	std::rotate(v.data() + cur - offset - 1, v.data() + cur - offset, v.data() + cur);

	// Expected: 0, 2, 3, 1
	for (const auto i : v)
	{
		std::cerr << i << " ";
	}
	std::cerr << std::endl;
}
