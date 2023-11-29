use crate::tcpflags;
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
use pnet::util;

use std::fmt::{self, Debug};
use std::net::Ipv4Addr;
const TCP_HEADER_SIZE: usize = 20;

// Clone traitが自動で実装される。後続の処理でclone()を使用する
#[derive(Clone)]
pub struct TCPPacket {
    // 構造体。
    buffer: Vec<u8>,
}

// TCPPacket構造体に対するメソッドの実装
impl TCPPacket {
    pub fn new(payload_len: usize) -> Self {
        Self {
            buffer: vec![0; TCP_HEADER_SIZE + payload_len],
        }
    }

    // source portを定義
    pub fn set_src(&mut self, port: u16) {
        self.buffer[0..2].copy_from_slice(&port.to_be_bytes())
    }

    // destination portを定義
    pub fn set_dest(&mut self, port: u16) {
        self.buffer[2..4].copy_from_slice(&port.to_be_bytes())
    }

    // TCPフラグを定義
    pub fn set_flag(&mut self, flag: u8) {
        self.buffer[13] = flag;
    }
}

// TCPPacket構造体にPacket traitを実装
impl Packet for TCPPacket {
    // TCPパケットの全データを返す
    fn packet(&self) -> &[u8] {
        &self.buffer
    }

    // TCPヘッダを除いたペイロードのデータを返す
    fn payload(&self) -> &[u8] {
        &self.buffer[TCP_HEADER_SIZE..]
    }
}
