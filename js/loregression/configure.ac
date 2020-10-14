AC_INIT(loregression)

AC_ARG_ENABLE(debug,
    AS_HELP_STRING([--enable-debug],
        [Determines whether to enable debug support.]),
,enable_debug=no)

DEBUG=""
if test "$enable_debug" = "yes"; then
    DEBUG="TRUE"
fi
AC_SUBST(DEBUG)

AC_CONFIG_FILES([config.mak])
AC_OUTPUT

dnl vim:set shiftwidth=4 softtabstop=4 expandtab:
