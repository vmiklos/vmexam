#include <boost/property_tree/json_parser.hpp>

int main()
{
	// {
	//    "values":
	//     [
	//         "foo",
	//         "bar"
	//     ],
	//     "foo": "bar"
	// }
	boost::property_tree::ptree aValues;
	boost::property_tree::ptree aChild1;
	aChild1.put("", "foo");
	boost::property_tree::ptree aChild2;
	aChild2.put("", "bar");
	boost::property_tree::ptree aChildren;
	aChildren.push_back(std::make_pair("", aChild1));
	aChildren.push_back(std::make_pair("", aChild2));
	aValues.add_child("values", aChildren);

	aValues.put("foo", "bar");

	std::stringstream aStream;
	boost::property_tree::write_json(aStream, aValues);
	std::cerr << aStream.str() << std::endl;
}
