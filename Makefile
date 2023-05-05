.PHONY: all info go

ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

#VG_SRC_ROOT := $(ROOT_DIR)krabcake-vg
VG_SRC_ROOT := krabcake-vg
# VG_LIBEXEC := $(ROOT_DIR)libexec/valgrind
VG_LIBEXEC := libexec/valgrind

VG_MAKEFILE_AMS := $(VG_SRC_ROOT)/Makefile.am $(VG_SRC_ROOT)/Makefile.all.am $(VG_SRC_ROOT)/Makefile.tool.am $(VG_SRC_ROOT)/Makefile.tool-tests.am $(VG_SRC_ROOT)/Makefile.vex.am
VG_TOOL_MAKEFILE_AMS := $(VG_SRC_ROOT)/krabcake/Makefile.am $(VG_SRC_ROOT)/coregrind/Makefile.am $(VG_SRC_ROOT)/include/Makefile.am $(VG_SRC_ROOT)/tests/Makefile.am $(VG_SRC_ROOT)/shared/Makefile.am $(VG_SRC_ROOT)/none/Makefile.am $(VG_SRC_ROOT)/lackey/Makefile.am

VG_INCL := $(VG_SRC_ROOT)/include
KC_INCL := $(VG_SRC_ROOT)/krabcake
KC_SRC  := $(VG_SRC_ROOT)/krabcake
KC_RS   = $(wildcard $(VG_SRC_ROOT)/krabcake/rs_hello/*) $(wildcard $(VG_SRC_ROOT)/krabcake/rs_hello/src/*)
KC_BINS := $(VG_LIBEXEC)/krabcake-x86-linux $(VG_LIBEXEC)/krabcake-amd64-linux

INCLUDE_OPTS := -I$(KC_INCL) -I$(VG_INCL)
INCLUDE_HDRS = $(VG_INCL)/valgrind.h $(KC_INCL)/krabcake.h $(wildcard $(VG_INCL)/*.h)

all: $(SB_RS_DBG) $(SB_RS_REL)

info: Makefile
	$(info ROOT_DIR is $(ROOT_DIR))

%/configure %/include/valgrind.h.in: %/configure.ac $(VG_MAKEFILE_AMS) $(VG_TOOL_MAKEFILE_AMS)
	cd $(VG_SRC_ROOT) && ./autogen.sh

$(VG_INCL)/valgrind.h: $(VG_SRC_ROOT)/configure $(VG_INCL)/valgrind.h.in
	cd $(VG_SRC_ROOT) && ./configure --prefix=$(ROOT_DIR)

go: $(KC_BINS)

$(KC_BINS): $(INCLUDE_HDRS) $(wildcard $(KC_SRC)/*) $(KC_RS)
	cd $(VG_SRC_ROOT) && $(MAKE) && $(MAKE) install
