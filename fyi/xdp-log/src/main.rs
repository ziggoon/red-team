use std::{mem, net::Ipv4Addr, ptr, str::from_utf8};

use aya::{include_bytes_aligned, maps::perf::AsyncPerfEventArray, Bpf, util::online_cpus};
use anyhow::Context;
use aya::programs::{Xdp, XdpFlags};
use aya_log::BpfLogger;
use bytes::BytesMut;
use clap::Parser;
use log::{info, warn};
use tokio::signal;

use hex;

use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};

use xdp_log_common::PacketBuffer;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "lo")]
    iface: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/xdp-log"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/xdp-log"
    ))?;
    if let Err(e) = BpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let program: &mut Xdp = bpf.program_mut("xdp").unwrap().try_into()?;
    program.load()?;
    program.attach(&opt.iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    let cpus = online_cpus()?;
    let num_cpus = cpus.len();
    println!("vCPUs: {}", num_cpus);
    
    let mut events = AsyncPerfEventArray::try_from(bpf.take_map("PACKET_DATA").unwrap())?;
    for cpu in cpus {
        let mut buf = events.open(cpu, None)?;

        tokio::task::spawn(async move {
            let mut bufs = (0..num_cpus)
                .map(|_| BytesMut::with_capacity(9000))
                .collect::<Vec<_>>();

            loop {
                let events = buf.read_events(&mut bufs).await.unwrap();
                for i in 0..events.read {
                    let buf = &mut bufs[i];

                    let hdr = unsafe { ptr::read_unaligned(buf.as_ptr() as *const PacketBuffer) };
                    let pkt_buf = buf.split().freeze().slice(mem::size_of::<PacketBuffer>()..mem::size_of::<PacketBuffer>() + hdr.size);
                    info!("userspace recevied packet of size {}", hdr.size);

                    let ethhdr = pkt_buf.slice(..EthHdr::LEN);
                    let ethhdr = unsafe { ptr::read_unaligned(ethhdr.as_ptr() as *const EthHdr) };
                    match ethhdr.ether_type {
                        EtherType::Ipv4 => {}
                        _ => continue,
                    }

                    let ipv4hdr = pkt_buf.slice(EthHdr::LEN..EthHdr::LEN + Ipv4Hdr::LEN);
                    let ipv4hdr = unsafe { ptr::read_unaligned(ipv4hdr.as_ptr() as *const Ipv4Hdr) };

                    let src_addr = u32::from_be(ipv4hdr.src_addr);
                    let src_addr = Ipv4Addr::from(src_addr);

                    let src_port = match ipv4hdr.proto {
                        IpProto::Tcp => {
                            let tcphdr = pkt_buf.slice(EthHdr::LEN + Ipv4Hdr::LEN..EthHdr::LEN + Ipv4Hdr::LEN + TcpHdr::LEN);
                            let tcphdr = unsafe { ptr::read_unaligned(tcphdr.as_ptr() as *const TcpHdr) };
                            u16::from_be(tcphdr.source)
                        }
                        IpProto::Udp => {
                            let udphdr = pkt_buf.slice(EthHdr::LEN + Ipv4Hdr::LEN..EthHdr::LEN + Ipv4Hdr::LEN + UdpHdr::LEN);
                            let udphdr = unsafe { ptr::read_unaligned(udphdr.as_ptr() as *const UdpHdr) };
                            u16::from_be(udphdr.source)
                        }
                        _ => continue,
                    };
                
                    let payload = pkt_buf.slice(EthHdr::LEN + Ipv4Hdr::LEN + TcpHdr::LEN..hdr.size);
                    info!("packet payload: {:?}", payload);
                    info!("src addr: {:?}, src port: {:?}", src_addr, src_port);
                }
            }

        });
    }
    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
