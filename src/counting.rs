use std::{
    sync::atomic::{AtomicU32, Ordering},
};

#[repr(transparent)]
#[derive(Debug)]
pub struct RefCounter {
    count: AtomicU32,
}
const _: () = crate::checks::assert_same_size_align::<RefCounter, AtomicU32>();

impl RefCounter {
    #[inline(always)]
    pub fn new(count: u32) -> Self {
        Self { count: AtomicU32::new(count) }
    }

    #[inline(always)]
    pub fn count(&self) -> u32 {
        self.count.load(Ordering::Acquire)
    }

    /// Increments the count and returns the previous count.
    #[inline(always)]
    pub fn increment(&self) -> u32 {
        self.count.fetch_add(1, Ordering::AcqRel)
    }

    // Decrements and returns Ok(new_count).
    pub fn decrement(&self) -> Result<u32, ()> {
        // cas loop for decrementing so that we do not decrement beyond 0.
        let mut count = self.count.load(Ordering::Relaxed);
        if count == 0 {
            return Err(());
        }
        loop {
            let decremented_count = count - 1;
            match self.count.compare_exchange(count, decremented_count, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(_) => return Ok(decremented_count),
                Err(0) => return Err(()),
                Err(previous) => count = previous,
            }
            std::hint::spin_loop();
        }
    }
}