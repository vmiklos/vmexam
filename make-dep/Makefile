all: hello/hello hello2/hello2

hello_objects = hello/lib.o hello/main.o
hello/hello: $(hello_objects)
	$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@

hello2_objects = hello2/lib2.o hello2/main2.o
hello2/hello2: $(hello2_objects)
	$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@

all_objects = $(hello_objects) $(hello2_objects)
SRCS = $(patsubst %.o,%.cxx,$(all_objects))

DEPDIR := .d
$(shell mkdir -p $(DEPDIR)/hello{,2} >/dev/null)
DEPFLAGS = -MT $@ -MMD -MP -MF $(DEPDIR)/$*.Td

COMPILE.cc = $(CXX) $(DEPFLAGS) $(CXXFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c

hello/%.o : hello/%.cxx
hello/%.o : hello/%.cxx $(DEPDIR)/hello/%.d
	$(COMPILE.cc) -DFOO $(OUTPUT_OPTION) $<
	mv -f $(DEPDIR)/$*.Td $(DEPDIR)/hello/$*.d

hello2/%.o : hello2/%.cxx
hello2/%.o : hello2/%.cxx $(DEPDIR)/hello2/%.d
	$(COMPILE.cc) -DBAR $(OUTPUT_OPTION) $<
	mv -f $(DEPDIR)/$*.Td $(DEPDIR)/hello2/$*.d

$(DEPDIR)/%.d: ;
.PRECIOUS: $(DEPDIR)/%.d

-include $(patsubst %,$(DEPDIR)/%.d,$(basename $(SRCS)))
