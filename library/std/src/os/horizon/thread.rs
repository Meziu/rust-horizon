//! Nintendo 3DS-specific extensions to primitives in the [`std::thread`] module.
//!
//! [`std::thread`]: crate::thread

#![unstable(feature = "horizon_thread_ext", issue = "none")]

/// Extensions on [`std::thread::Builder`] for the Nintendo 3DS.
///
/// [`std::thread::Builder`]: crate::thread::Builder
pub trait ThreadBuilderExt: Sized {
    /// Sets the priority level for the new thread.
    ///
    /// Low values gives the thread higher priority. For userland apps, this has
    /// to be within the range of 0x18 to 0x3F inclusive. The main thread usually
    /// has a priority of 0x30, but not always.
    fn priority(mut self, priority: i32) -> Self;

    /// Sets the ID of the processor the thread should be run on. Threads on the 3DS are only
    /// preemptive if they are on the system core. Otherwise they are cooperative (must yield to let
    /// other threads run).
    ///
    /// Processor IDs are labeled starting from 0. On Old3DS it must be <2, and
    /// on New3DS it must be <4. Pass -1 to execute the thread on all CPUs and
    /// -2 to execute the thread on the default CPU (set in the application's Exheader).
    ///
    /// *Processor #0 is the application core. It is always possible to create a thread on this
    /// core.
    /// *Processor #1 is the system core. If the CPU time limit is set, it is possible
    /// to create a single thread on this core.
    /// *Processor #2 is New3DS exclusive. Normal applications can create threads on
    /// this core if the exheader kernel flags bitmask has 0x2000 set.
    /// *Processor #3 is New3DS exclusive. Normal applications cannot create threads
    /// on this core.
    fn ideal_processor(mut self, ideal_processor: i32) -> Self;
}

impl ThreadBuilderExt for crate::thread::Builder {
    fn priority(mut self, priority: i32) -> Self {
        self.native_options.priority = Some(priority);
        self
    }

    fn ideal_processor(mut self, ideal_processor: i32) -> Self {
        self.native_options.ideal_processor = Some(ideal_processor);
        self
    }
}
