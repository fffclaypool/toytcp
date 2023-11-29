use crate::packet::TCPPacket;
use crate::socket::{SockID, Socket, TcpStatus};
use crate::tcpflags;
use anyhow::{Context, Result};
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
use pnet::transport::{self, TransportChannelType};
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex, RwLock, RwLockWriteGuard};
use std::time::{Duration, SystemTime};
use std::{cmp, ops::Range, str, thread};

const UNDETERMINED_IP_ADDR: std::net::Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const UNDETERMINED_PORT: u16 = 0;
const MAX_TRANSMITTION: u8 = 5;
const RETRANSMITTION_TIMEOUT: u64 = 3;
const MSS: usize = 1460;
const PORT_RANGE: Range<u16> = 40000..60000;

pub struct TCP {
    sockets: HashMap<SockID, Socket>,
}

impl TCP {
    // TCPインスタンスを返す
    pub fn new() -> Self {
        let sockets = HashMap::new();
        let tcp = Self { sockets };
        tcp
    }

    // 未使用のポート番号を返すべきだが, 現在は固定値を返す
    fn select_unused_port(&self, rng: &mut ThreadRng) -> Result<u16> {
        Ok(33445)
    }

    // 指定されたリモートアドレスとポートに接続するためのソケットを作成する
    pub fn connect(&self, addr: Ipv4Addr, port: u16) -> Result<SockID> {
        // デフォルトの乱数生成器を初期化する
        let mut rng = rand::thread_rng();
        // 新しいソケットを作成する
        let mut socket = Socket::new(
            get_source_addr_to(addr)?,
            addr,
            self.select_unused_port(&mut rng)?,
            port,
        )?;
        // SYNフラグを持つTCPパケットを送信する
        socket.send_tcp_packet(tcpflags::SYN, &[])?;
        let sock_id = socket.get_sock_id();
        // 成功した場合は, ソケットのIDを返す
        Ok(sock_id)
    }
}

// 特定の宛先アドレスに接続するために使用するソースIPv4アドレスを返す。現在は固定値としている
fn get_source_addr_to(addr: Ipv4Addr) -> Result<Ipv4Addr> {
    Ok("10.0.0.1".parse().unwrap())
}
