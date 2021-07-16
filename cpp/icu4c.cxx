#include <algorithm>
#include <iostream>
#include <vector>

#include <unicode/ucol.h>
#include <unicode/ustring.h>

std::string getSortKey(const std::string& bytes)
{
    UErrorCode status = U_ZERO_ERROR;
    UCollator* collator = ucol_open("hu_HU.UTF-8", &status);
    if (U_FAILURE(status))
    {
        std::cerr << "ucol_open() failed" << std::endl;
    }

    std::vector<UChar> string(bytes.size());
    int32_t destLength = 0;
    u_strFromUTF8(string.data(), string.size(), &destLength, bytes.data(),
                  bytes.size(), &status);
    if (destLength > string.size())
    {
        string.resize(destLength);
        u_strFromUTF8(string.data(), string.size(), &destLength, bytes.data(),
                      bytes.size(), &status);
        if (U_FAILURE(status))
        {
            std::cerr << "u_strFromUTF8() failed" << std::endl;
        }
    }

    std::vector<uint8_t> result;
    destLength = ucol_getSortKey(collator, string.data(), string.size(),
                                 result.data(), result.size());
    if (destLength > result.size())
    {
        result.resize(destLength);
        ucol_getSortKey(collator, string.data(), string.size(), result.data(),
                        result.size());
        if (!string.empty() && destLength == 0)
        {
            std::cerr << "ucol_getSortKey() failed" << std::endl;
        }
    }

    ucol_close(collator);

    return std::string((const char*)result.data(), result.size());
}

int main()
{
    std::vector<std::string> strings = {"Kőpor", "Kórház"};
    std::sort(strings.begin(), strings.end(),
              [](const std::string& a, const std::string& b) -> bool {
                  return getSortKey(a) < getSortKey(b);
              });
    for (const auto& string : strings)
    {
        std::cerr << string << " ";
    }
    std::cerr << std::endl;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
