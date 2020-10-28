//! Telemetry client for F1 game by Codemasters
//!
//! # Example
//!
//! ```rust
//! use f1_telmetry_client::{Telemetry, packet::Packet};
//! use async_std::task;
//!
//! fn main() {
//!     task::block_on(async {
//!         let telemetry = Telemetry::new("127.0.0.1", 20777).await.unwrap();
//!
//!         loop {
//!             match telemetry.next().await {
//!                 Ok(packet) => {
//!                     match packet {
//!                         Packet::F12020(result) => {
//!                             println!("Result {:?}", result);
//!                         }
//!                         _ => unimplemented!(),
//!                     }
//!                 }
//!                 Err(e) => {
//!                     eprintln!("Error {}", e)
//!                 }
//!             }
//!         }
//!     })
//! }
//! ```

use async_std::io::{Cursor, Error};
use async_std::net::{IpAddr, SocketAddr, UdpSocket};

use byteorder_async::{LittleEndian, ReaderToByteOrder};
use std::str::FromStr;

pub mod f1_2020;
pub mod packet;

pub struct Telemetry(UdpSocket);

impl Telemetry {
    pub async fn new(ip: &str, port: u16) -> Result<Self, Error> {
        let ip = IpAddr::from_str(ip).expect("Invalid ip address");
        let socket_addrs = SocketAddr::new(ip, port);
        let socket = UdpSocket::bind(socket_addrs).await?;

        Ok(Telemetry(socket))
    }

    pub async fn next(&self) -> Result<packet::Packet, Error> {
        let mut buf = vec![0; 2048];
        let (size, _) = self.0.recv_from(&mut buf).await?;
        let mut cursor = Cursor::new(buf);

        let packet_format = cursor
            .clone()
            .byte_order()
            .read_u16::<LittleEndian>()
            .await?;
        match packet_format {
            2020 => {
                let result = f1_2020::packet::parse_f12020(&mut cursor, size).await?;
                Ok(packet::Packet::F12020(result))
            }
            2019 => unimplemented!(),
            2018 => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::net::UdpSocket;
    use async_std::task::spawn;
    use byteorder_async::WriteBytesExt;

    fn send() {
        let handle = spawn(async {
            let socket = UdpSocket::bind("127.0.0.1:8080").await.unwrap();
            let mut send_buf = Vec::with_capacity(2048);
            send_buf.write_u16::<LittleEndian>(2020).unwrap();
            send_buf.write_u8(1).unwrap();
            send_buf.write_u8(2).unwrap();
            send_buf.write_u8(3).unwrap();
            send_buf.write_u8(0).unwrap();
            send_buf
                .write_u64::<LittleEndian>(u64::max_value())
                .unwrap();
            send_buf.write_f32::<LittleEndian>(1.0).unwrap();
            send_buf
                .write_u32::<LittleEndian>(u32::max_value())
                .unwrap();
            send_buf.write_u8(19).unwrap();
            send_buf.write_u8(255).unwrap();
            socket.send_to(&*send_buf, "127.0.0.1:20777").await.unwrap();
        });

        drop(handle);
    }

    #[async_std::test]
    async fn test_telemetry_next() {
        send();

        let client = Telemetry::new("127.0.0.1", 20777).await.unwrap();
        let result = client.next().await.map_err(|e| e.kind());
        assert_eq!(result, Err(async_std::io::ErrorKind::InvalidData));
    }
}
