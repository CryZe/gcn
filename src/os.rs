#[allow(non_snake_case)]
extern "C" {
    #[link_name = "OSGetCurrentThread"]
    pub fn get_current_thread() -> *const u8;
    #[link_name = "OSIsThreadTerminated"]
    pub fn is_thread_terminated(this: *const u8) -> bool;
    #[link_name = "OSCreateThread"]
    pub fn create_thread(
        this: *mut u8,
        entry: extern "C" fn(*mut u8) -> *mut u8,
        arg: *mut u8,
        stack: *mut u8,
        stack_size: usize,
        priority: i32,
        attr: i16,
    ) -> bool;
    #[link_name = "OSResumeThread"]
    pub fn resume_thread(this: *const u8) -> i32;
    #[link_name = "OSSuspendThread"]
    pub fn suspend_thread(this: *const u8) -> i32;
    #[link_name = "OSJoinThread"]
    pub fn join_thread(this: *const u8, ret_value: *mut *mut u8) -> bool;
    #[link_name = "OSYieldThread"]
    pub fn yield_thread();
    #[link_name = "OSInitMutex"]
    pub fn init_mutex(this: *mut u8);
    #[link_name = "OSLockMutex"]
    pub fn lock_mutex(this: *const u8);
    #[link_name = "OSUnlockMutex"]
    pub fn unlock_mutex(this: *const u8);
    #[link_name = "OSTryLockMutex"]
    pub fn try_lock_mutex(this: *const u8) -> bool;
    #[link_name = "OSInitCond"]
    pub fn init_cond(this: *mut u8);
    #[link_name = "OSWaitCond"]
    pub fn wait_cond(this: *const u8, mutex: *const u8);
    #[link_name = "OSSignalCond"]
    pub fn signal_cond(this: *const u8);
    #[link_name = "OSGetTime"]
    pub fn get_time() -> i64;
    #[link_name = "OSReport"]
    pub fn report(text: *const u8);
    #[link_name = "OSPanic"]
    pub fn panic(file: *const u8, line: i32, message: *const u8);
}
