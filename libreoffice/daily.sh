if [ -n "$1" ]; then
	date=$1
else
	date=$(date +%Y-%m-%d)
fi
daily_dir=$HOME/git/libreoffice/daily

# this is here to avoid committing to a detached head
cd $daily_dir
branch=$(git symbolic-ref -q HEAD)
[ "${branch##*/}" == "master" ] || git checkout master
# in case yesterday's build was done on an other machine
git pull -r
cd -

rm -rf $daily_dir/opt
make cmd cmd="ooinstall --strip $daily_dir/opt"
git log -10 > $daily_dir/build-info.txt
cd $daily_dir
git add -A
git commit -m "$date"
if git config remote.origin.url | grep -q bitbucket; then
	git push
fi
