#include <stddef.h>
#include <string.h>

void xmlSecError(const char* file, int line, const char* func,
                 const char* errorObject, const char* errorSubject, int reason,
                 const char* msg, ...)
{
}

#define XMLSEC_ERRORS_HERE __FILE__, __LINE__, ""
#define XMLSEC_ERRORS_R_XMLSEC_FAILED 1
#define XMLSEC_ERRORS_NO_MESSAGE " "

#define xmlSecInternalError(errorFunction, errorObject)                        \
    xmlSecError(XMLSEC_ERRORS_HERE, (const char*)(errorObject),                \
                (errorFunction), XMLSEC_ERRORS_R_XMLSEC_FAILED,                \
                XMLSEC_ERRORS_NO_MESSAGE)

int foo() { return -1; }

int outer(int x, int y) { return -1; }

int main()
{
    int ret;

    // OK
    ret = foo();
    if (ret < 0)
    {
        xmlSecInternalError("foo", NULL);
    }

    // OK
    ret = outer(1, strlen("x"));
    if (ret < 0)
    {
        xmlSecInternalError("outer", NULL);
    }

    // KO
    ret = foo();
    if (ret < 0)
    {
        xmlSecInternalError("bar", NULL);
    }
}
