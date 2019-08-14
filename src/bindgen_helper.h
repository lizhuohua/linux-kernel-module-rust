#include <linux/fs.h>
#include <linux/uaccess.h>
#include <linux/sysctl.h>
#include <linux/spinlock.h>
#include <linux/spinlock_types.h>
#include <linux/mutex.h>
#include <linux/netdevice.h>
#include <linux/mii.h>
#include <linux/usb.h>
#include <linux/usb/usbnet.h>
#include <linux/of_net.h>
#include <linux/umh.h>
#include <linux/sched.h>
#include <linux/module.h>

int usbnet_read_cmd(struct usbnet *dev, u8 cmd, u8 reqtype, u16 value,
                    u16 index, void *data, u16 size);

void bug_helper(void);
