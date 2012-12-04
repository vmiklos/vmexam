if [ -n "$1" ]; then
	date=$1
else
	date=$(date +%Y-%m-%d)
fi
daily_dir=$HOME/git/libreoffice/daily
rm -rf $daily_dir/opt
make cmd cmd="ooinstall $daily_dir/opt"
git log -10 > $daily_dir/build-info.txt
cd $daily_dir
git add -A
git commit -m "$date"
