#include <iomanip>
#include <iostream>

#include <cert.h>

void test(const char* string)
{
    CERTName* name = CERT_AsciiToName(string);
    if (name)
    {
        std::cerr << "CERT_AsciiToName() succeeded for '" << string << "'"
                  << std::endl;

        PRArenaPool* arena = PORT_NewArena(DER_DEFAULT_CHUNKSIZE);
        SECItem* derName = SEC_ASN1EncodeItem(arena, nullptr, name,
                                              SEC_ASN1_GET(CERT_NameTemplate));
        std::cerr << "derName->len is " << derName->len << std::endl;
        for (size_t i = 0; i < derName->len; ++i)
        {
            std::cerr << std::setw(2) << std::setfill('0') << std::hex
                      << int(*(derName->data + i));
        }
        std::cerr << std::endl;
        PORT_FreeArena(arena, PR_FALSE);

        CERT_DestroyName(name);
    }
    else
        std::cerr << "CERT_AsciiToName() failed for '" << string << "'"
                  << std::endl;
}

int main()
{
    test("C=HU,L=Budapest,O=NISZ Nemzeti Infokommunikációs Szolgáltató "
         "Zrt.,CN=Állampolgári Tanúsítványkiadó - Qualified Citizen "
         "CA,2.5.4.97=VATHU-10585560");
    // test("C=HU,L=Budapest,O=NISZ Nemzeti Infokommunikációs Szolgáltató
    // Zrt.,CN=Állampolgári Tanúsítványkiadó - Qualified Citizen CA");

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
