#![no_std]
#![no_main]

use aya_bpf::{bindings::xdp_action, macros::{map, xdp}, maps::PerfEventArray, programs::XdpContext};
use aya_log_ebpf::info;

use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};

use xdp_log_common::PacketBuffer;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[map]
pub static PACKET_DATA: PerfEventArray<PacketBuffer> = PerfEventArray::new(0);

#[xdp]
pub fn xdp_firewall(ctx: XdpContext) -> u32 {
    match try_xdp_firewall(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_xdp_firewall(ctx: XdpContext) -> Result<u32, ()> {
    let len = ctx.data_end() - ctx.data();
    PACKET_DATA.output(&ctx, &PacketBuffer { size: len }, len as u32);

    Ok(xdp_action::XDP_PASS)
}
