use std::{
    sync::atomic::{AtomicU32, Ordering},
};

#[repr(transparent)]
#[derive(Debug)]
pub struct RefCounter {
    count: AtomicU32,
}

impl RefCounter {
    #[inline(always)]
    pub fn new() -> Self {
        Self { count: AtomicU32::new(0) }
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

    // Decrements and returns Ok(true) when the count has reached 0.
    pub fn decrement(&self) -> Result<bool, ()> {
        // cas loop for decrementing so that we do not decrement beyond 0.
        let mut count = self.count.load(Ordering::Relaxed);
        loop {
            if count == 0 {
                return Err(());
            }
            match self.count.compare_exchange(count, count-1, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(previous) => return Ok(previous == 1),
                Err(previous) => {
                    count = previous;
                    std::hint::spin_loop();
                },
            }
        }
    }
}