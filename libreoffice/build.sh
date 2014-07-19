# this can do a build after a 'git clean -x -d -f'
(cd external; ln -s ../../tarballs)
ln -s ~/git/vmexam/libreoffice/autogen.input
ln -s ~/git/vmexam/libreoffice/up.sh
. ./up.sh
