#include <boost/property_tree/json_parser.hpp>

int main()
{
	std::ifstream file("test.json");
	if (!file)
		return 1;
	std::stringstream aStream;
	aStream << file.rdbuf();
	file.close();
	boost::property_tree::ptree aTree;
	boost::property_tree::read_json(aStream, aTree);
	for (boost::property_tree::ptree::value_type& rValue : aTree.get_child("values"))
	{
		std::cerr << "part is " << rValue.second.get<std::string>("part") << std::endl;
		std::cerr << "rect is " << rValue.second.get<std::string>("rect") << std::endl;
	}
}
