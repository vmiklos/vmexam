use Alpm::Core;

sub getowner
{
	my $what = shift;
	$i = Alpm::Core::alpm_db_getpkgcache($db);
	while($i)
	{
		$pkg = Alpm::Core::void_to_PM_PKG(Alpm::Core::alpm_list_getdata($i));
		$j = Alpm::Core::void_to_PM_LIST(Alpm::Core::alpm_pkg_getinfo($pkg, $Alpm::Core::PM_PKG_FILES));
		while($j)
		{
			if(Alpm::Core::void_to_char(Alpm::Core::alpm_list_getdata($j)) eq $what)
			{
				return Alpm::Core::void_to_char(Alpm::Core::alpm_pkg_getinfo($pkg, $Alpm::Core::PM_PKG_NAME));
			}
			$j = Alpm::Core::alpm_list_next($j);
		}
		$i = Alpm::Core::alpm_list_next($i);
	}
}

Alpm::Core::alpm_initialize("/");
$db = Alpm::Core::alpm_db_register('local');

print getowner("usr/bin/pacman") . "\n";
print getowner("lib/libc.so.6") . "\n";
