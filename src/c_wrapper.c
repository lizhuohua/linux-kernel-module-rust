#include <linux/spinlock.h>
#include <linux/uaccess.h>

long copy_to_user_wrapper(void __user *to, const void *from, unsigned long n) {
    return _copy_to_user(to, from, n);
}

void spin_lock_init_wrapper(spinlock_t *lock) { spin_lock_init(lock); }

void spin_lock_wrapper(spinlock_t *lock) { spin_lock(lock); }
void spin_unlock_wrapper(spinlock_t *lock) { spin_unlock(lock); }
