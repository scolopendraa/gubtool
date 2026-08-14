CXX := clang-cl
SYSROOT := $(HOME)/.xwin
LIBFFI := $(HOME)/.libffi

CXXFLAGS := /winsysroot $(SYSROOT) \
            /LD \
            /MD \
            /EHsc \
            /std:c++20

LDFLAGS := -fuse-ld=lld

.PHONY: all x64 x86 clean

all: x64 x86

x64:
	$(CXX) --target=x86_64-pc-windows-msvc \
		$(CXXFLAGS) \
		/I"$(LIBFFI)/x64/include" \
		dllmain.cpp \
		port.cpp \
		handle.cpp \
		/Fe:../../gubtool-core/src/sys/resources/WinWorkerThread64.dll \
		$(LDFLAGS) \
		/link \
		/LIBPATH:"$(LIBFFI)/x64" \
		libffi.lib \
		/NOIMPLIB

x86:
	$(CXX) --target=i686-pc-windows-msvc \
		$(CXXFLAGS) \
		/I"$(LIBFFI)/x86/include" \
		dllmain.cpp \
		port.cpp \
		handle.cpp \
		/Fe:../../gubtool-core/src/sys/resources/WinWorkerThread32.dll \
		$(LDFLAGS) \
		/link \
		/LIBPATH:"$(LIBFFI)/x86" \
		libffi.lib \
		/NOIMPLIB

clean:
	rm -rf build
