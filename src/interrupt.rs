use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicBool};

pub struct Interrupt(Arc<AtomicBool>);

impl Interrupt {
    pub fn setup() -> anyhow::Result<Self> {
        let flag = Arc::new(AtomicBool::new(false));
        let interrupted = Self(flag.clone());
        ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst))?;
        Ok(interrupted)
    }

    pub fn should_continue(&self) -> bool {
        !self.0.load(Ordering::SeqCst)
    }
}
