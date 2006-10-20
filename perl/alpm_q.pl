#!/usr/bin/perl -w

use Alpm::Core;

Alpm::Core::alpm_initialize("/");
my $db = Alpm::Core::alpm_db_register("local");
my $lp = Alpm::Core::alpm_db_getpkgcache($db);
my $pkg = Alpm::Core::void_to_PM_PKG(Alpm::Core::alpm_list_getdata($lp));
print Alpm::Core::void_to_char(Alpm::Core::alpm_pkg_getinfo($pkg, $Alpm::Core::PM_PKG_NAME)) . "\n";
