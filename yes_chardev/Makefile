obj-m := yes_chardev.o
yes_chardev-objs := target/x86_64-linux-kernel-module/debug/libyes_chardev.a
EXTRA_LDFLAGS += --entry=init_module

all:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR)

clean:
	$(MAKE) -C /lib/modules/$(shell uname -r)/build M=$(CURDIR) clean
