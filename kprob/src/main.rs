#![no_std]
#![no_main]

use aya_bpf::{
    macros::{kprobe, map},
    maps::HashMap,
    programs::ProbeContext,
};

#[panic_handler]
fn panic_impl(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[map(name = "counter")]
static mut CNT: HashMap<u32, u32> = HashMap::with_max_entries(10, 0);

#[kprobe(name = "tcp_connect")]
pub fn tcp_connect(_ctx: ProbeContext) {
    //
}

#[no_mangle]
fn __tcp_connect() {}
