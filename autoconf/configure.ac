dnl Process this file with autoconf to produce a configure script.
AC_INIT(hello, 0.1, vmiklos@frugalware.org)

AC_PROG_INSTALL
AC_CHECK_PROG(DARCS, darcs, darcs, [missing darcs])

for prog in "$DARCS"; do
  case $prog in
    missing\ *)
      prog=`echo $prog |sed 's/.* \(.*\)/\1/'`
      AC_MSG_ERROR([Could not find required program: $prog])
      ;;
  esac
done

AC_CONFIG_FILES([Makefile])
AC_OUTPUT
