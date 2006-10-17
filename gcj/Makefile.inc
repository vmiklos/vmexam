# l33t autodetect ;)

ifneq ($(shell which ecj 2>/dev/null),)
JAVAC = ecj
else
JAVAC = javac
endif

ifneq ($(shell which fastjar 2>/dev/null),)
JAR = fastjar
else
JAR = jar
endif

ifneq ($(shell which gij 2>/dev/null),)
JAVA = gij
else
JAVA = java
endif

all: $(MYJAR)

%.class: %.java
	$(JAVAC) $<

%.jar:
	$(JAR) cf $@ $^

run: $(MYJAR)
	$(JAVA) $(basename $^)

clean:
	rm -fv *.jar *.class

# vim: ft=make
