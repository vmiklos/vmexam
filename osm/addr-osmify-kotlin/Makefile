# Java 11 seems to be problematic.
export JAVA_HOME = /usr/lib64/jvm/java-1.8.0

all:
	mvn assembly:assembly -DdescriptorId=jar-with-dependencies

run:
	java -jar target/addr-osmify-1.0-SNAPSHOT-jar-with-dependencies.jar

check: all
	ktlint "src/**/*.kt"
