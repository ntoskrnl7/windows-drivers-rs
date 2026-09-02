// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

#[cfg(test)]
mod tests {
    use wdk::sync::{PushLock, RwLock, RwSpinLock};
    #[allow(unused_imports)]
    use wdk_sys::test_stubs as _;

    #[test]
    fn rw_lock_read_and_write_guards_access_value() {
        let lock = RwLock::try_new(1_u32).expect("ERESOURCE initialization should succeed");

        assert_eq!(*lock.read(), 1);

        {
            let mut value = lock.write();
            *value = 2;
        }

        assert_eq!(
            *lock.try_read().expect("shared acquisition should succeed"),
            2
        );

        {
            let mut value = lock
                .try_write()
                .expect("exclusive acquisition should succeed");
            *value += 1;
        }

        assert_eq!(*lock.read(), 3);
    }

    #[test]
    fn rw_lock_get_mut_accesses_value_without_locking() {
        let mut lock = RwLock::try_new(1_u32).expect("ERESOURCE initialization should succeed");

        *lock.get_mut() = 7;

        assert_eq!(*lock.read(), 7);
    }

    #[test]
    fn rw_lock_try_methods_reject_recursive_acquisition() {
        let lock = RwLock::try_new(1_u32).expect("ERESOURCE initialization should succeed");

        let read_guard = lock.read();
        assert!(lock.try_read().is_none());
        assert!(lock.try_write().is_none());
        drop(read_guard);

        let write_guard = lock.write();
        assert!(lock.try_read().is_none());
        assert!(lock.try_write().is_none());
        drop(write_guard);

        assert!(lock.try_read().is_some());
    }

    #[test]
    #[should_panic(expected = "recursive RwLock acquisition")]
    fn rw_lock_read_panics_on_recursive_acquisition() {
        let lock = RwLock::try_new(1_u32).expect("ERESOURCE initialization should succeed");
        let _write_guard = lock.write();

        let _recursive_guard = lock.read();
    }

    #[test]
    #[should_panic(expected = "recursive RwLock acquisition")]
    fn rw_lock_write_panics_on_recursive_acquisition() {
        let lock = RwLock::try_new(1_u32).expect("ERESOURCE initialization should succeed");
        let _read_guard = lock.read();

        let _recursive_guard = lock.write();
    }

    #[test]
    fn push_lock_read_and_write_guards_access_value() {
        let lock = PushLock::new(1_u32);

        assert_eq!(*lock.read(), 1);

        {
            let mut value = lock.write();
            *value = 4;
        }

        assert_eq!(*lock.read(), 4);
    }

    #[test]
    fn push_lock_get_mut_accesses_value_without_locking() {
        let mut lock = PushLock::new(1_u32);

        *lock.get_mut() = 9;

        assert_eq!(*lock.read(), 9);
    }

    #[test]
    fn rw_spin_lock_read_and_write_guards_access_value() {
        let lock = RwSpinLock::new(1_u32);

        assert_eq!(*lock.read(), 1);

        {
            let mut value = lock.write();
            *value = 5;
        }

        assert_eq!(*lock.read(), 5);
    }

    #[test]
    fn rw_spin_lock_get_mut_accesses_value_without_locking() {
        let mut lock = RwSpinLock::new(1_u32);

        *lock.get_mut() = 11;

        assert_eq!(*lock.read(), 11);
    }

    #[test]
    fn rw_spin_lock_dpc_level_guards_access_value() {
        let lock = RwSpinLock::new(1_u32);

        // SAFETY: The test stubs do not inspect or modify IRQL, so they model
        // the caller already running at DISPATCH_LEVEL.
        let value = unsafe { lock.read_at_dpc_level() };
        assert_eq!(*value, 1);
        drop(value);

        // SAFETY: The test stubs do not inspect or modify IRQL, so they model
        // the caller already running at DISPATCH_LEVEL.
        let mut value = unsafe { lock.write_at_dpc_level() };
        *value = 13;
        drop(value);

        assert_eq!(*lock.read(), 13);
    }
}
