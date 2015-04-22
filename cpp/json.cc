#include <sstream>
#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/json_parser.hpp>

// Example on how to read/write JSON using Boost.PropertyTree.

int main()
{
    boost::property_tree::ptree aTree;
    aTree.put(boost::property_tree::ptree::path_type("SearchItem.SearchString/type", '/'), "string");
    aTree.put(boost::property_tree::ptree::path_type("SearchItem.SearchString/value", '/'), "him");
    aTree.put(boost::property_tree::ptree::path_type("SearchItem.Backward/type", '/'), "boolean");
    aTree.put(boost::property_tree::ptree::path_type("SearchItem.Backward/value", '/'), false);
    std::stringstream aStream;
    boost::property_tree::write_json(aStream, aTree);
    std::cout << "json is " << aStream.str() << std::endl;

    boost::property_tree::read_json(aStream, aTree);

    for (const std::pair<std::string, boost::property_tree::ptree>& rPair : aTree)
    {
        const std::string& rType = rPair.second.get<std::string>("type");
        const std::string& rValue = rPair.second.get<std::string>("value");
        std::cout << "key is " << rPair.first << ", type is " << rType << ", value is " << rValue << std::endl;
    }

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
