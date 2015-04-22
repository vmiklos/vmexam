#include <sstream>
#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/json_parser.hpp>

// Example on how to produce JSON using Boost.PropertyTree.

int main()
{
    boost::property_tree::ptree aTree;
    aTree.put(boost::property_tree::ptree::path_type("SearchItem.SearchString/type", '/'), "string");
    aTree.put(boost::property_tree::ptree::path_type("SearchItem.SearchString/value", '/'), "him");
    aTree.put(boost::property_tree::ptree::path_type("SearchItem.Backward/type", '/'), "boolean");
    aTree.put(boost::property_tree::ptree::path_type("SearchItem.Backward/value", '/'), false);
    std::stringstream aStream;
    boost::property_tree::write_json(aStream, aTree);
    std::cout << "debug, json is '" << aStream.str() << "'";
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
