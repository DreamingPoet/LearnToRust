use std::thread;
pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {

    /// Create a new TreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The 'new' function will panic is the size is zero.
    pub fn new(size:usize) -> ThreadPool{
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for i in 0..size{
            
        }

        ThreadPool{ threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // FnOnce 代表一个闭包函数，函数的定义后面带(), 返回类型可以省略
        {

        }
}