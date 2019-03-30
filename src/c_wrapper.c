#include <linux/mutex.h>
#include <linux/spinlock.h>
#include <linux/uaccess.h>
#include <linux/skbuff.h>

long copy_to_user_wrapper(void __user *to, const void *from, unsigned long n) {
    return _copy_to_user(to, from, n);
}

void spin_lock_init_wrapper(spinlock_t *lock) { spin_lock_init(lock); }
void spin_lock_wrapper(spinlock_t *lock) { spin_lock(lock); }
void spin_unlock_wrapper(spinlock_t *lock) { spin_unlock(lock); }

void mutex_init_wrapper(struct mutex *lock) { mutex_init(lock); }
void mutex_lock_wrapper(struct mutex *lock) { mutex_lock(lock); }
void mutex_unlock_wrapper(struct mutex *lock) { mutex_unlock(lock); }

void skb_set_tail_pointer_wrapper(struct sk_buff *skb, const int offset) {
    skb_set_tail_pointer(skb, offset);
}
