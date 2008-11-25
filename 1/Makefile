SOURCES = $(wildcard *.java)
CLASSES = $(patsubst %.java,%.class,$(SOURCES))

all: program.jar
	
program.jar: $(CLASSES)
	jar cmf manifest.mf program.jar *.class

%.class: %.java
	javac $(SOURCES)

clean:
	rm -f *.class

run: program.jar
	java -jar program.jar hf01_pelda_input.txt

output.txt: run

test: output.txt
	diff -u hf01_pelda_output.txt output.txt
