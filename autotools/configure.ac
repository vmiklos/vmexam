AC_INIT([amhello], [1.0], [bug-automake@gnu.org])
AM_INIT_AUTOMAKE([-Wall -Werror foreign])
AC_PROG_CXX
AC_CONFIG_FILES([
    Makefile
    source/Makefile
])

dnl Use ccache automatically.
AC_PATH_PROG([CCACHE],[ccache],[not found])
if test "$CCACHE" != "not found"; then
    CXX="$CCACHE $CXX"
fi

AC_OUTPUT

dnl vim:set shiftwidth=4 softtabstop=4 expandtab:
