obj-m := sync_example.o
ifeq ($(TARGET),armv7l-linux-kernel-module)
sync_example-objs := target/$(TARGET)/debug/libsync_example.a ../src/c_wrapper.o
else
sync_example-objs := target/x86_64-linux-kernel-module/debug/libsync_example.a ../src/c_wrapper.o
endif
EXTRA_LDFLAGS += --entry=init_module

ifeq ($(TARGET),armv7l-linux-kernel-module)
CMD_LINE = ARCH=arm CROSS_COMPILE=$(CROSS)
else
KDIR = /lib/modules/$(shell uname -r)/build
CMD_LINE =
endif

all:
	$(MAKE) $(CMD_LINE) -C $(KDIR) M=$(CURDIR)

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
