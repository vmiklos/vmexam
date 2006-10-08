ALPM_JAR = /usr/share/java/alpm.jar

main:
	CLASSPATH=$$CLASSPATH:$(ALPM_JAR) javac main.java

run:
	CLASSPATH=$$CLASSPATH:$(ALPM_JAR) java main

clean:
	rm -f main.class
