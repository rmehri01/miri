//@compile-flags: -Zmiri-preemption-rate=0.0 -Zmiri-disable-weak-memory-emulation
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering};
use std::thread;

fn convert(a: &AtomicU16) -> &[AtomicU8; 2] {
    unsafe { std::mem::transmute(a) }
}

// We can't allow mixed-size accesses; they are not possible in C++ and even
// Intel says you shouldn't do it.
fn main() {
    let a = AtomicU16::new(0);
    let a16 = &a;
    let a8 = convert(a16);

    thread::scope(|s| {
        s.spawn(|| {
            a16.store(1, Ordering::SeqCst);
        });
        s.spawn(|| {
            a8[0].store(1, Ordering::SeqCst);
            //~^ ERROR: Data race detected between (1) 2-byte Atomic Store on thread `<unnamed>` and (2) 1-byte (different-size) Atomic Store on thread `<unnamed>`
        });
    });
}
