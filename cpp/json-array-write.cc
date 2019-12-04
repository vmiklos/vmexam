#include <boost/property_tree/json_parser.hpp>
#include <iostream>

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
	aChild1.put("part", "1");
	aChild1.put("rect", "x, y, w, h");
	boost::property_tree::ptree aChild2;
	aChild2.put("part", "2");
	aChild2.put("rect", "x2, y2, w2, h2");
	boost::property_tree::ptree aChildren;
	aChildren.push_back(std::make_pair("", aChild1));
	aChildren.push_back(std::make_pair("", aChild2));
	aValues.add_child("values", aChildren);

	aValues.put("foo", "bar");

	std::stringstream aStream;
	boost::property_tree::write_json(aStream, aValues);
	std::cerr << aStream.str() << std::endl;
}
