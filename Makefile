.PHONY: all info go

ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

VG_SRC_ROOT := $(ROOT_DIR)krabcake-vg
VG_LIBEXEC := $(ROOT_DIR)libexec/valgrind

VG_INCL := $(VG_SRC_ROOT)/include
KC_INCL := $(VG_SRC_ROOT)/krabcake
KC_SRC  := $(VG_SRC_ROOT)/krabcake
KC_BINS := $(VG_LIBEXEC)/krabcake-x86-linux $(VG_LIBEXEC)/krabcake-amd64-linux

INCLUDE_OPTS := -I$(KC_INCL) -I$(VG_INCL)
INCLUDE_HDRS := $(VG_INCL)/valgrind.h $(KC_INCL)/*.h $(VG_INCL)/*.h

all: sb_in_c

info: Makefile
	$(info ROOT_DIR is $(ROOT_DIR))

$(VG_INCL)/valgrind.h: $(VG_INCL)/valgrind.h.in $(VG_SRC_ROOT)/configure.ac $(VG_SRC_ROOT)/Makefile.am $(VG_SRC_ROOT)/Makefile.*.am $(VG_SRC_ROOT)/*/Makefile.am
	cd $(VG_SRC_ROOT) && ./autogen.sh
	cd $(VG_SRC_ROOT) && ./configure --prefix=$(ROOT_DIR)

sb_in_c: sb_in_c.c $(INCLUDE_HDRS)
	$(CC) -o $@ $< $(INCLUDE_OPTS)

go: sb_in_c $(KC_BINS)
	export VALGRIND_LIB=$(VG_LIBEXEC); ./bin/valgrind --tool=krabcake ./sb_in_c

$(KC_BINS): $(INCLUDE_HDRS) $(KC_SRC)/*
	cd $(VG_SRC_ROOT) && $(MAKE)
	cd $(VG_SRC_ROOT) && $(MAKE) install
