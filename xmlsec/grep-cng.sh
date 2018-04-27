for i in $(cat lo-use-cng.txt)
do
	cd ~/git/xmlsec
	if ! git grep -q $i; then
		echo "$i KO"
	else
		echo "$i OK"
	fi
done
