#![no_std]
#![no_main]

use aya_bpf::{
    macros::{map, xdp},
    programs::XdpContext,
};

use aya_bpf::maps::HashMap;
use aya_log_ebpf::info;

#[map(name = "TEST_MAP")]
static mut TEST_MAP: HashMap<u64, u64> = HashMap::<u64, u64>::with_max_entries(1024, 0);

#[xdp(name="xdp")]
pub fn xdp(ctx: XdpContext) -> u32 {
    let key: u64 = 0;
    unsafe {
        match TEST_MAP.get_ptr(&key) {
            None => info!(&ctx, "None"),
            Some(result) => info!(&ctx, "Some"),
        }
        match TEST_MAP.remove(&key) {
            Ok(_) => info!(&ctx, "Ok"),
            Err(error) => info!(&ctx, "Error: {}", error),
        }
    }
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
