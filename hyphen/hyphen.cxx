/*
 * Copyright 2021 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#include <iostream>
#include <memory>
#include <string>
#include <vector>

#include <hyphen.h>

struct DictDeleter
{
    inline void operator()(HyphenDict* dict) { hnj_hyphen_free(dict); }
};

using ScopedHyphenDict = std::unique_ptr<HyphenDict, DictDeleter>;

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        std::cerr << "missing dictionary file" << std::endl;
        return 1;
    }

    const char* dict_path = argv[1];
    ScopedHyphenDict dict(hnj_hyphen_load(dict_path));
    if (dict == nullptr)
    {
        std::cerr << "failed to load the dictionary" << std::endl;
        return 1;
    }

    if (argc < 3)
    {
        std::cerr << "missing word" << std::endl;
        return 1;
    }

    std::string word(argv[2]);

    // See
    // <https://github.com/hunspell/hyphen/blob/73dd2967c8e1e4f6d7334ee9e539a323d6e66cbd/example.c#L136>.
    std::vector<char> hyphens(word.size() + 5);

    // See
    // <https://github.com/hunspell/hyphen/blob/73dd2967c8e1e4f6d7334ee9e539a323d6e66cbd/example.c#L156>.
    std::vector<char> hword(word.size() * 2);

    char** rep = nullptr;

    int* pos = nullptr;

    int* cut = nullptr;

    if (hnj_hyphen_hyphenate2(dict.get(), word.data(), word.size(),
                              hyphens.data(), hword.data(), &rep, &pos, &cut))
    {
        std::cerr << "hnj_hyphen_hyphenate2() failed" << std::endl;
        return 1;
    }

    std::cout << std::string(hword.data(), hword.size()) << std::endl;

    if (rep)
    {
        for (size_t i = 0; i < word.size(); i++)
        {
            if (rep[i])
            {
                std::free(rep[i]);
            }
        }
        std::free(rep);
        std::free(pos);
        std::free(cut);
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
