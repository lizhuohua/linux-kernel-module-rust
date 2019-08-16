# Copyright (C) 2019 Alex Gaynor, Geoffrey Thomas, and other project authors
# 
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License along
# with this program; if not, write to the Free Software Foundation, Inc.,
# 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.<Paste>

ifneq ($(KERNELRELEASE),)
obj-m += dummy.o
clean-files := dummy.c

# Some systems for installing kernel headers (e.g. Debian's) happen to
# trigger the out-of-tree build code because the kernel headers directly
# actually just recursively invokes another non-arch-specific one. This
# means that they already generate absolute paths for -I by using the
# flags/addtree make functions.  Some (e.g. Ubuntu's) do not, and
# generate relative paths. We want absolute paths, but we cannot force
# the out-of-tree build code because it won't work on Debian-style
# kernel headers directory (it will look in the mostly-empty kernel
# headers directory instead of the actual one). So we steal the addtree
# and flags functions from scripts/Kbuild.include, and use them _after_
# the build system has generated paths - if any remaining paths are
# relative, we make them absolute with respect to CURDIR. (Unlike the
# upstream addtree function, we prefix -I./foo. We also need to fix
# -include ./include/linux/kconfig.h)
our_addtree = $(if $(patsubst -I%,%,$(1)), \
$(if $(filter-out -I/% -I../%,$(1)),$(patsubst ./%,$(CURDIR)/%,$(patsubst -I%,-I$(CURDIR)/%,$(1))),$(1)),$(1))
our_flags = $(foreach o,$($(1)),$(call our_addtree,$(o)))
# Clang doesn't support these arguments, just ignoring them seems to be ok
# clang_unsupported = -mapcs -mno-sched-prolog -mno-thumb-interwork
# @echo $(NOSTDINC_FLAGS) $(call our_flags,LINUXINCLUDE) $(filter-out $(clang_unsupported),$(__c_flags)) $(modkern_cflags)

$(M)/dummy.c:
	@echo $(NOSTDINC_FLAGS) $(call our_flags,LINUXINCLUDE) $(__c_flags) $(modkern_cflags)
	@touch $@

.PHONY: $(M)/dummy.c
else

ifeq ($(TARGET),armv7l-linux-kernel-module)
CMD_LINE = ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf-
else
KDIR = /lib/modules/$(shell uname -r)/build
endif

all:
	$(MAKE) CC=clang HOSTCC=clang $(CMD_LINE) -C $(KDIR) M=$(CURDIR)
clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
endif
