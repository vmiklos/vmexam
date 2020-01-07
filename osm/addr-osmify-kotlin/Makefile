all:
	mvn assembly:assembly -DdescriptorId=jar-with-dependencies

run:
	java -jar target/addr-osmify-1.0-SNAPSHOT-jar-with-dependencies.jar

check:
	ktlint "src/**/*.kt"
