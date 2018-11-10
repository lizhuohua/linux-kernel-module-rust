#include <linux/uaccess.h>

long copy_to_user_wrapper(void __user *to, const void *from, unsigned long n) {
    return _copy_to_user(to, from, n);
}
