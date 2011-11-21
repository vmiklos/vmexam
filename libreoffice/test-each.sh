#!/bin/bash

# This script puts each test file to the indeterminate separately, so in case
# multiple files crash the import filter, we can detect more than a single
# crash with a single run.

reproducers=$(mktemp)
cd /home/vmiklos/git/libreoffice/master/writerfilter
count=0
for i in ../../lo-test-files/writer/from-caolan/*
do
	count=$((count + 1))
	echo "Testing $i (#$count)"
	rm -f qa/cppunittests/rtftok/data/indeterminate/*
	cp $i qa/cppunittests/rtftok/data/indeterminate/
	if ! make -sr -j4 dbglevel=2; then
		echo $i >> $reproducers
	fi
done
if [ -s $reproducers ]; then
	echo "Failed, reproducers:"
	cat $reproducers
else
	echo "$count files passed the test."
fi
rm -f $reproducers
