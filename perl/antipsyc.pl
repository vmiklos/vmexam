use strict;
use vars qw($VERSION %IRSSI);

use Irssi;
$VERSION = '0.1';
%IRSSI = (
	authors     => 'Miklos Vajna',
	contact     => 'vmiklos@vmiklos.hu',
	name        => 'antipsyc',
	description => 'This one actually makes your life really happier.',
	license     => 'GPL',
);

my $stripped_in  = 0;

sub psyc_in {
	if(Irssi::settings_get_bool('psyc_strip_in') && !$stripped_in) {
		my $emitted_signal = Irssi::signal_get_emitted();
		my ($server, $data, $nick, $address, $junk) = @_;
		$nick =~ s/net\/irc\/user#//;
		$stripped_in=1;
		Irssi::signal_emit("$emitted_signal", $server, $data, $nick, $address, $junk);
		Irssi::signal_stop();
		$stripped_in=0;
	}
}

Irssi::settings_add_bool('lookandfeel', 'psyc_strip_in', 1);
Irssi::signal_add_first('server event', 'psyc_in');
