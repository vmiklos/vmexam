all:
	mvn assembly:assembly -DdescriptorId=jar-with-dependencies

run:
	java -jar target/addr-osmify-1.0-SNAPSHOT-jar-with-dependencies.jar

# alias pmd="$PWD/pmd-bin-6.17.0/bin/run.sh pmd"
check:
	pmd -d src/main/java/ -R rulesets/java/quickstart.xml -f text

docs:
	mvn javadoc:javadoc

tags:
	ctags --fields=+iaS --extra=+q -R --totals=yes src
