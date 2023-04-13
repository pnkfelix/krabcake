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

SB_RS_CRATE_NAME := sb_rs_port
SB_RS_DIR := $(ROOT_DIR)/$(SB_RS_CRATE_NAME)
SB_RS_DBG := $(SB_RS_DIR)/target/debug/$(SB_RS_CRATE_NAME)
SB_RS_REL := $(SB_RS_DIR)/target/release/$(SB_RS_CRATE_NAME)

all: $(SB_RS_DBG) $(SB_RS_REL)

info: Makefile
	$(info ROOT_DIR is $(ROOT_DIR))

$(VG_INCL)/valgrind.h: $(VG_INCL)/valgrind.h.in $(VG_SRC_ROOT)/configure.ac $(VG_SRC_ROOT)/Makefile.am $(VG_SRC_ROOT)/Makefile.*.am $(VG_SRC_ROOT)/*/Makefile.am
	cd $(VG_SRC_ROOT) && ./autogen.sh
	cd $(VG_SRC_ROOT) && ./configure --prefix=$(ROOT_DIR)

$(SB_RS_REL): $(SB_RS_DIR)/* $(SB_RS_DIR)/src/*
	cd $(SB_RS_DIR) && cargo build --release

$(SB_RS_DBG): $(SB_RS_DIR)/* $(SB_RS_DIR)/src/*
	cd $(SB_RS_DIR) && cargo build

taret/debug/sb_in_rust: sb_rs_port/* sb_rs_port/src/*

go: $(SB_RS_DBG) $(SB_RS_REL) $(KC_BINS)
	export VALGRIND_LIB=$(VG_LIBEXEC); ./bin/valgrind --tool=krabcake $(SB_RS_DBG)
	export VALGRIND_LIB=$(VG_LIBEXEC); ./bin/valgrind --tool=krabcake $(SB_RS_REL)

$(KC_BINS): $(INCLUDE_HDRS) $(KC_SRC)/* $(KC_RS)
	cd $(VG_SRC_ROOT) && $(MAKE)
	cd $(VG_SRC_ROOT) && $(MAKE) install
