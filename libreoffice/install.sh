#!/bin/bash

if [ -z "$1" ]; then
	echo "usage: $0 <prefix to install to>"
	exit 1
fi
#rpm --dbpath $1/.RPM_OFFICE_DATABASE --query -a
#rpm --upgrade --ignoresize --nodeps -vh --relocate /opt/libreoffice4.1=$1/opt/libreoffice4.1 --dbpath $1/.RPM_OFFICE_DATABASE RPMS/*.rpm
CWD=$(pwd)
cd $1
for i in $CWD/RPMS/*.rpm
do
	rpm2cpio $i |cpio -idmv
done
