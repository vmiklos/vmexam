class S
{
    int m_nX = 0;

  public:
    /// This can be const.
    int getX() { return m_nX; }

    /// This is const already.
    int getXConst() const { return m_nX; }

    /// Assignment to primitive type -> can't be const.
    void setX(int nX) { m_nX = nX; }
};

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
