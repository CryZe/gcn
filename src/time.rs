extern "C" {
    #[link_name = "OSGetTick"]
    pub fn ticks() -> u32;
}

pub const BUS_CLOCK: u64 = 162000000;
pub const TIMER_CLOCK: u64 = BUS_CLOCK / 4;

#[inline]
pub fn ticks_as_nanoseconds() -> u64 {
    unsafe { ticks() as u64 * 8000 / (TIMER_CLOCK / 125000) }
}
