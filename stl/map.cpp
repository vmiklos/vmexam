#include <iostream>
#include <map>

int main()
{
	std::map<char,int> m;
	std::map<char,int>::iterator it;

	m.insert ( std::pair<char,int>('a',1) );
	m.insert ( std::pair<char,int>('b',2) );
	for ( it=m.begin() ; it != m.end(); it++ )
		std::cout << (*it).first << " => " << (*it).second << std::endl;
}
