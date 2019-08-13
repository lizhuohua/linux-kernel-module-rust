obj-m := yes_chardev.o
ifeq ($(RELEASE),1)
	BUILD = release
else
	BUILD = debug
endif

ifeq ($(TARGET),armv7l-linux-kernel-module)
yes_chardev-objs := target/$(TARGET)/$(BUILD)/libyes_chardev.a ../src/c_wrapper.o
else
yes_chardev-objs := target/x86_64-linux-kernel-module/$(BUILD)/libyes_chardev.a ../src/c_wrapper.o
endif
EXTRA_LDFLAGS += --entry=init_module

ifeq ($(TARGET),armv7l-linux-kernel-module)
CMD_LINE = ARCH=arm CROSS_COMPILE=$(CROSS)
else
KDIR = /lib/modules/$(shell uname -r)/build
endif

all:
	$(MAKE) $(CMD_LINE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
