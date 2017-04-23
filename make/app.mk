#
# @section License
#
# The MIT License (MIT)
#
# Copyright (c) 2017, Erik Moqvist
#
# Permission is hereby granted, free of charge, to any person
# obtaining a copy of this software and associated documentation
# files (the "Software"), to deal in the Software without
# restriction, including without limitation the rights to use, copy,
# modify, merge, publish, distribute, sublicense, and/or sell copies
# of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be
# included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
# NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
# BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
# ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
#
# This file is part of the Rafiki project.
#

BUILDDIR ?= build/$(BOARD)
OBJDIR = $(BUILDDIR)/obj
DEPSDIR = $(BUILDDIR)/deps
GENDIR = $(BUILDDIR)/gen
MAIN_C =
SRC += main.rs
RUST_SRC += $(filter %.rs,$(SRC))
RUST_OBJ = $(patsubst %,$(OBJDIR)%,$(abspath $(RUST_SRC:%.rs=%.o)))
OBJ += $(RUST_OBJ)

LIB += \
	core \
	rafiki

LDFLAGS += -L$(BUILDDIR)
RSFLAGS += \
	--crate-type lib \
	-L$(BUILDDIR) \
	--target thumbv7em-none-eabi

include $(SIMBA_ROOT)/make/app.mk

define RUST_COMPILE_template
-include $(patsubst %.rs,$(DEPSDIR)%.o.dep,$(abspath $1))
$(patsubst %.rs,$(OBJDIR)%.o,$(abspath $1)): $1 $(GENDIR)/rafiki.rs $(BUILDDIR)/librafiki.rlib
	@echo "RUSTC $1"
	mkdir -p $(OBJDIR)$(abspath $(dir $1))
	mkdir -p $(DEPSDIR)$(abspath $(dir $1))
	mkdir -p $(GENDIR)$(abspath $(dir $1))
	rustc $(RSFLAGS) -o $$@ $$<
endef
$(foreach file,$(RUST_SRC),$(eval $(call RUST_COMPILE_template,$(file))))

$(GENDIR)/rafiki.rs:
	@echo "BINDGEN rafiki"
	mkdir -p $(GENDIR)
	bindgen \
	    --builtins \
	    --no-layout-tests \
	    --use-core \
	    --ctypes-prefix "::ctypes" \
	    --no-unstable-rust \
	    --with-derive-default \
	    --output $(GENDIR)/rafiki.bindgen.rs \
	    $(SIMBA_ROOT)/src/simba.h -- $(INC:%=-I%) $(CDEFS:%=-D%)
	cat $(GENDIR)/rafiki.bindgen.rs \
	| grep -v "pub type time_t = __time_t;" \
	| grep -v "pub type timer_t = __timer_t;" > $(GENDIR)/rafiki.rs
	rm $(GENDIR)/rafiki.bindgen.rs

-include $(BUILDDIR)/core.d
$(BUILDDIR)/libcore.rlib:
	@echo "RUSTC libcore"
	mkdir -p $(BUILDDIR)
	rustc \
	    --crate-name core \
	    --crate-type lib \
	    --emit=dep-info,link \
	    -C debuginfo=0 \
	    -O \
	    --out-dir $(BUILDDIR) \
	    --target thumbv7em-none-eabi \
	    $(HOME)/.rustup/toolchains/nightly-*/lib/rustlib/src/rust/src/libcore/lib.rs

-include $(BUILDDIR)/rafiki.d
$(BUILDDIR)/librafiki.rlib: $(BUILDDIR)/libcore.rlib $(GENDIR)/rafiki.rs
	@echo "RUSTC librafiki"
	mkdir -p $(BUILDDIR)
	env BUILDDIR=$(shell readlink -f $(GENDIR)) rustc \
	    --crate-name rafiki \
	    --crate-type lib \
	    --emit=dep-info,link \
	    -C debuginfo=0 \
	    -O \
	    --out-dir $(BUILDDIR) \
	    --target thumbv7em-none-eabi \
	    -L$(BUILDDIR) \
	    $(RAFIKI_ROOT)/src/lib.rs

$(BUILDDIR)/libcore.a: $(BUILDDIR)/libcore.rlib
	cp $< $@

$(BUILDDIR)/librafiki.a: $(BUILDDIR)/librafiki.rlib
	cp $< $@

generate: $(BUILDDIR)/librafiki.a $(BUILDDIR)/libcore.a
