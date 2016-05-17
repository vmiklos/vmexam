for i in $(git ls-files|grep java$); do
	if ! grep -q vim: $i; then
		echo '/* -*- Mode: Java; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */' > tmp
		cat $i >> tmp
		mv -f tmp $i
		if [ -n "`tail -n 1 $i`" ] ; then
			echo >> $i
		fi
		echo '/* vim:set shiftwidth=4 softtabstop=4 expandtab: */' >> $i
	fi
done
