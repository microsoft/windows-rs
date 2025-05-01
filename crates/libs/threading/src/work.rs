use super::*;

pub struct Work(PTP_WORK);

impl Work {
    pub fn new<F: FnMut() + Send + 'static>(f: F) {
        
    }
}