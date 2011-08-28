#!/bin/bash

if [ -z "$1" ]; then
	echo "usage: $0 <prefix to install to>"
	exit 1
fi
rpm --dbpath $1/.RPM_OFFICE_DATABASE --query -a
rpm --upgrade --ignoresize --nodeps -vh --relocate /opt=$1/opt --dbpath $1/.RPM_OFFICE_DATABASE RPMS/*.rpm
