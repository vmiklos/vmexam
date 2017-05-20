class S
{
    int m_nX = 0;

  public:
    /// This can be const, we want to find this one.
    int getX() { return m_nX; }

    /// This is const already.
    int getXConst() const { return m_nX; }

    /// This is static already.
    static int getXStatic() { return 0; }

    /// Assignment to primitive type -> can't be const.
    void setX(int nX) { m_nX = nX; }
};

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
