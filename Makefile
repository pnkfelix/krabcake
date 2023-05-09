.PHONY: all info go test

# In case you are not familiar with `make`: the .PHONY stuff is a way
# to tag a target T so that if you ever do `make T`, the recipe for T
# will run *regardless* of whether a file named T happens to exist.
# (Normally, if the file T existed and the rule had no prerequisites
# to compare against, then the recipe would not be run.)



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

# This is just some space to do some makefile debugging; run `make
# info` to see it print out.
info: Makefile
	$(info ROOT_DIR is $(ROOT_DIR))

# pnkfelix could not figure out a clean way to get the Makefile to
# automatically update the submodules for us; the files themselves
# being absent before you run the command is almost certainly part of
# the reason this is tricky. So, instead of worrying about making that
# step automatic, lets just issue a nice message when it is clearly
# the missing step that the user need to do.
$(VG_SRC_ROOT)/configure.ac $(VG_MAKEFILE_AMS) $(VG_TOOL_MAKEFILE_AMS):
	@echo 'You need to run `git submodule update --init` before you run `make`.'
	@false

### Note: if I ever get tempted to try to encode recipes that generate
### more than one file, the *only* way to properly do it is via
### wildcare rules, e.g.
#
# %/target1 %/target2: %/source-file
#
### (At some point I had erronously thought that autogen.sh was
### creating both the configure *and* the valgrind.h.in, which led
### down some very bad/confusing paths.)

# This is not quite right; it runs autogen.sh which rebuilds a bunch more stuff.
%/configure: %/configure.ac
	echo Rerunning autogen.sh to rebuild $@ due to $?
	cd $* && ./autogen.sh

%/krabcake/Makefile.in: %/krabcake/Makefile.am
	echo Rerunning automake to rebuild $@ due to $?
	cd $* && automake -a

# $(VG_MAKEFILE_AMS) $(VG_TOOL_MAKEFILE_AMS)

### %/aclocal.m4: %/configure.ac
### 	cd $* && aclocal
### 
### %/Makefile.in: %/Makefile.am
### 	cd $* && autoheader
### 	cd $* && automake -a
### 
### %/configure: %/configure.ac
### 	cd $* && autoconf

$(VG_SRC_ROOT)/include/valgrind.h: $(VG_SRC_ROOT)/configure $(VG_SRC_ROOT)/include/valgrind.h.in
	cd $(VG_SRC_ROOT) && ./configure --prefix=$(ROOT_DIR)

test: $(KC_BINS)
	cd kc && cargo run

# Originally `make go` had a demo file associated with it. And I want to
# make that the case again, once we have the whole pipeline set up with
# a patched rustc
go: $(KC_BINS)

$(KC_BINS): $(INCLUDE_HDRS) $(wildcard $(KC_SRC)/*) $(KC_RS)
	cd $(VG_SRC_ROOT) && $(MAKE) && $(MAKE) install
