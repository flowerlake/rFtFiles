pub mod util;
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        assert!(size < 10);
        ThreadPool
    }
}
