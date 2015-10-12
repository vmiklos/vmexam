# Script to allow bibisecting tiled rendering problems.
#export MASTER=~/git/libreoffice/daily
#INSTDIR=opt
export MASTER=~/git/libreoffice/master
INSTDIR=instdir
SYSTEMPLATE=`pwd`/systemplate
ROOTFORJAILS=`pwd`/jails

rm -Rf ${SYSTEMPLATE} ${ROOTFORJAILS}
./loolwsd-systemplate-setup ${SYSTEMPLATE} ${MASTER}/$INSTDIR
mkdir -p ${ROOTFORJAILS}
make clean-cache
./loolwsd --systemplate=${SYSTEMPLATE} --lotemplate=${MASTER}/$INSTDIR --childroot=${ROOTFORJAILS} --numprespawns=1
