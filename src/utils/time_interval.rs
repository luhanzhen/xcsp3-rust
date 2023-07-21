/*
 * <p>@project_name: xcsp3-rust
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/7/21 13:46
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn 
 * </p>
 * <p>@version: 1.0
 * </p>
  * <p>@description: 
 * </p>
 */
pub mod xcsp3_utils {
    use std::time::{Duration, Instant};

    pub struct TimeInterval
    {
        start: Instant,
    }

    impl TimeInterval {
        pub fn new() -> Self
        {
            Self
            {
                start: Instant::now()
            }
        }

        pub fn get(&self) -> Duration
        {
            self.start.elapsed()
        }
    }

    impl Default for TimeInterval {
        fn default() -> Self {
            Self::new()
        }
    }
}
 
 
 
 
 
 