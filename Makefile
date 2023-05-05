.PHONY: all info go

ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

VG_SRC_ROOT := $(ROOT_DIR)krabcake-vg
VG_LIBEXEC := $(ROOT_DIR)libexec/valgrind

VG_INCL := $(VG_SRC_ROOT)/include
KC_INCL := $(VG_SRC_ROOT)/krabcake
KC_SRC  := $(VG_SRC_ROOT)/krabcake
KC_RS   := $(VG_SRC_ROOT)/krabcake/rs_hello/* $(VG_SRC_ROOT)/krabcake/rs_hello/src/*
KC_BINS := $(VG_LIBEXEC)/krabcake-x86-linux $(VG_LIBEXEC)/krabcake-amd64-linux

INCLUDE_OPTS := -I$(KC_INCL) -I$(VG_INCL)
INCLUDE_HDRS := $(VG_INCL)/valgrind.h $(KC_INCL)/*.h $(VG_INCL)/*.h

all: $(SB_RS_DBG) $(SB_RS_REL)

info: Makefile
	$(info ROOT_DIR is $(ROOT_DIR))

$(VG_INCL)/valgrind.h.in:
	git submodule update --init

$(VG_INCL)/valgrind.h: $(VG_INCL)/valgrind.h.in $(VG_SRC_ROOT)/configure.ac $(VG_SRC_ROOT)/Makefile.am $(VG_SRC_ROOT)/Makefile.*.am $(VG_SRC_ROOT)/*/Makefile.am
	cd $(VG_SRC_ROOT) && ./autogen.sh
	cd $(VG_SRC_ROOT) && ./configure --prefix=$(ROOT_DIR)

go: $(KC_BINS)

$(KC_BINS): $(INCLUDE_HDRS) $(KC_SRC)/* $(KC_RS)
	cd $(VG_SRC_ROOT) && $(MAKE)
	cd $(VG_SRC_ROOT) && $(MAKE) install
