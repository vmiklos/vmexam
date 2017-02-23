#include <iostream>

#include <cert.h>

void test(const char* string)
{
    CERTName* name = CERT_AsciiToName(string);
    if (name)
    {
        std::cerr << "CERT_AsciiToName() succeeded for '" << string << "'" << std::endl;
        CERT_DestroyName(name);
    }
    else
        std::cerr << "CERT_AsciiToName() failed for '" << string << "'" << std::endl;
}

int main()
{
    test("C=HU,L=Budapest,O=NISZ Nemzeti Infokommunikációs Szolgáltató Zrt.,CN=Állampolgári Tanúsítványkiadó - Qualified Citizen CA,2.5.4.97=VATHU-10585560");
    test("C=HU,L=Budapest,O=NISZ Nemzeti Infokommunikációs Szolgáltató Zrt.,CN=Állampolgári Tanúsítványkiadó - Qualified Citizen CA");

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
