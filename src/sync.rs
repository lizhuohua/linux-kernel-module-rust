use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut, Drop};
use crate::bindings;
use crate::println;

extern "C" {
    pub fn spin_lock_init_wrapper(lock: *mut bindings::spinlock_t);
    pub fn spin_lock_wrapper(lock: *mut bindings::spinlock_t);
    pub fn spin_unlock_wrapper(lock: *mut bindings::spinlock_t);

    pub fn mutex_init_wrapper(lock: *mut bindings::mutex);
    pub fn mutex_lock_wrapper(lock: *mut bindings::mutex);
    pub fn mutex_unlock_wrapper(lock: *mut bindings::mutex);
}

pub struct Spinlock<T: ?Sized> {
    lock: UnsafeCell<bindings::spinlock_t>,
    data: UnsafeCell<T>,
}

pub struct SpinlockGuard<'a, T: ?Sized + 'a> {
    lock: &'a mut bindings::spinlock_t,
    data: &'a mut T,
}

unsafe impl<T: ?Sized + Send> Sync for Spinlock<T> {}
unsafe impl<T: ?Sized + Send> Send for Spinlock<T> {}

impl<T> Spinlock<T> {
    pub fn new(user_data: T) -> Spinlock<T> {
        let mut lock = bindings::spinlock_t::default();
        unsafe {
            spin_lock_init_wrapper(&mut lock);
        }
        Spinlock {
            lock: UnsafeCell::new(lock),
            data: UnsafeCell::new(user_data),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<T> {
        unsafe {
            spin_lock_wrapper(self.lock.get());
            println!("Spinlock is locked!");
        }
        SpinlockGuard {
            lock: unsafe { &mut *self.lock.get() },
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<'a, T: ?Sized> Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { spin_unlock_wrapper(self.lock) }
        println!("Spinlock is dropped!");
    }
}

pub struct Mutex<T: ?Sized> {
    lock: UnsafeCell<bindings::mutex>,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a mut bindings::mutex,
    data: &'a mut T,
}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(user_data: T) -> Mutex<T> {
        let mut lock = bindings::mutex::default();
        unsafe {
            mutex_init_wrapper(&mut lock);
        }
        Mutex {
            lock: UnsafeCell::new(lock),
            data: UnsafeCell::new(user_data),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        unsafe {
            mutex_lock_wrapper(self.lock.get());
            println!("Mutex is locked!");
        }
        MutexGuard {
            lock: unsafe { &mut *self.lock.get() },
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { mutex_unlock_wrapper(self.lock) }
        println!("Mutex is dropped!");
    }
}

pub fn drop<T>(_x: T) { }
