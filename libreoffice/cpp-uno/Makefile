WORKDIR = workdir
SP :=
SP +=

ifneq ($(OS),Windows_NT)
LO_HOME = /opt/libreoffice
OO_SDK_NAME = libreoffice$(shell grep ProductKey $(LO_HOME)/program/bootstraprc|sed 's/ProductKey=LibreOffice //')_sdk
DEFINES = -DLINUX -DUNX -DCPPU_ENV=gcc3
INCLUDES = -I$(LO_HOME)/sdk/include -I$(WORKDIR)
LIBS = -L$(LO_HOME)/sdk/lib -L$(LO_HOME)/ure/lib -Wl,-rpath,$(LO_HOME)/ure/lib -luno_cppuhelpergcc3 -luno_cppu -luno_salhelpergcc3 -luno_sal -lxmlreader
ifneq ($(findstring 4.0,$(OO_SDK_NAME)),4.0)
LIBS += -lreg -lunoidl -lstore
endif

test: test.cxx $(WORKDIR)/com/sun/star/uno/Exception.hpp Makefile
	g++ -Wall -o test $(DEFINES) $(INCLUDES) $(LIBS) test.cxx

$(WORKDIR)/com/sun/star/uno/Exception.hpp:
	$(LO_HOME)/sdk/bin/cppumaker -Gc $(if $(findstring 4.0,$(OO_SDK_NAME)),-BUCR) -O./$(WORKDIR) $(LO_HOME)/ure/share/misc/types.rdb $(LO_HOME)/program/types/offapi.rdb

run: test
	./test -env:URE_MORE_TYPES="file://$(LO_HOME)/program/types/offapi.rdb" "file://$(LO_HOME)/sdk/examples/cpp/DocumentLoader/test.odt"
else
LO_HOME = $(OFFICE_HOME)
LO_URL = $(subst $(SP),%20,$(subst $(strip \ ),/,$(LO_HOME)))
COMPATH = C:/PROGRA~1/MICROS~2.0/VC
WINDOWS_SDK_HOME = C:/PROGRA~1/MICROS~2/Windows/v7.1
CXXFLAGS = -DWIN32 -DWNT -DCPPU_ENV=msci -nologo $(WD) -EHa -I$(COMPATH)/include "-I$(LO_HOME)/sdk/include" -I$(WORKDIR)
LDFLAGS = -nologo -LIBPATH:$(COMPATH)/lib -LIBPATH:$(WINDOWS_SDK_HOME)/lib
LIBS = "$(LO_HOME)/sdk/lib/icppuhelper.lib" "$(LO_HOME)/sdk/lib/icppu.lib" "$(LO_HOME)/sdk/lib/isalhelper.lib" "$(LO_HOME)/sdk/lib/isal.lib"

test.exe: test.o
	$(COMPATH)/bin/link.exe $(LDFLAGS) $(LIBS) test.o -out:test.exe

test.o: test.cxx $(WORKDIR)/com/sun/star/uno/Exception.hpp Makefile
	$(COMPATH)/bin/cl.exe $(CXXFLAGS) -c test.cxx -Fotest.o

$(WORKDIR)/com/sun/star/uno/Exception.hpp:
	cppumaker -Gc $(if $(findstring 4.0,$(OO_SDK_NAME)),-BUCR) -O./$(WORKDIR) "$(LO_HOME)/URE/misc/types.rdb" "$(LO_HOME)/program/types/offapi.rdb"

run: test.exe
	./test.exe -env:URE_MORE_TYPES="file:///$(LO_URL)/program/types/offapi.rdb" "file:///$(LO_URL)/sdk/examples/cpp/DocumentLoader/test.odt"
endif

clean:
	rm -rf $(WORKDIR) test test.exe test.o
