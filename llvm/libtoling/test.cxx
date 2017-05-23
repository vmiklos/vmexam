#include <string>

#define FOO()                                                                  \
    int getXMacro() { return m_nX; }

class S
{
    int m_nX = 0;
    std::string m_aX;

  public:
    /// This can be const, only reads a member.
    int getX() { return m_nX; }

    /// This can be const, only calls const.
    void callsConst() { getXConst(); }

    /// This is const already.
    int getXConst() const { return m_nX; }

    /// This is static already.
    static int getXStatic() { return 0; }

    /// Assignment to primitive type -> can't be const.
    void setX(int nX) { m_nX = nX; }

    /// Assignment to non-primitive type -> can't be const.
    void setXString(const std::string& rX) { m_aX = rX; }

    /// Virtual -> can't be const.
    virtual int getXVirtual() { return 0; }

    /// Calls non-const -> can't be const.
    void callsNonconst() { setX(0); }

    FOO();
};

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
