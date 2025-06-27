all:
	mvn package

run:
	java -jar target/addr-osmify-1.0-SNAPSHOT-jar-with-dependencies.jar

check: all
	ktlint "src/**/*.kt"

check-outdated:
	mvn versions:display-dependency-updates
