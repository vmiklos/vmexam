#include <sstream>
#include <Poco/JSON/Object.h>
#include <Poco/JSON/Parser.h>

// Example on how to tweak JSON using Poco.

int main()
{
    // Extracts the value of the rendering key from the input, recursively.
    std::string options = "{\"rendering\":{\".uno:HideWhitespace\":{\"type\":\"boolean\",\"value\":\"false\"}}}";
    Poco::JSON::Parser parser;
    Poco::Dynamic::Var var = parser.parse(options);
    Poco::JSON::Object::Ptr object = var.extract<Poco::JSON::Object::Ptr>();
    std::string renderingOptions = object->get("rendering").toString();
    std::cerr << "debug, renderingOptions is '"<<renderingOptions<<"'" << std::endl;

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
