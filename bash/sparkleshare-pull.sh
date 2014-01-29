# Simple stupid script to pull all SparkleShare repos before starting the GUI,
# which likes to crash if there is too much stuff to pull...

for i in *
do
	cd $i || continue
	git pull
	cd -
done
