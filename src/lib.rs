#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

include! {concat!(env!("OUT_DIR"), "/bindings.rs")}

pub const _ENetProtocolFlag_ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE: u32 = 1 << 7;
pub const _ENetProtocolFlag_ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED: u32 = 1 << 6;
pub const _ENetProtocolFlag_ENET_PROTOCOL_HEADER_FLAG_SENT_TIME: u32 = 1 << 14;
pub const _ENetProtocolFlag_ENET_PROTOCOL_HEADER_FLAG_MASK: u32 =
    _ENetProtocolFlag_ENET_PROTOCOL_HEADER_FLAG_SENT_TIME;
pub const _ENetProtocolFlag_ENET_PROTOCOL_HEADER_SESSION_MASK: u32 = 3 << 12;
pub const _ENetProtocolFlag_ENET_PROTOCOL_HEADER_SESSION_SHIFT: u32 = 12;

pub const _ENetSocketWait_ENET_SOCKET_WAIT_NONE: u32 = 0;
pub const _ENetSocketWait_ENET_SOCKET_WAIT_SEND: u32 = 1 << 0;
pub const _ENetSocketWait_ENET_SOCKET_WAIT_RECEIVE: u32 = 1 << 1;
pub const _ENetSocketWait_ENET_SOCKET_WAIT_INTERRUPT: u32 = 1 << 2;

pub const _ENetPacketFlag_ENET_PACKET_FLAG_NONE: u32 = 0;
pub const _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE: u32 = 1 << 0;
pub const _ENetPacketFlag_ENET_PACKET_FLAG_UNSEQUENCED: u32 = 1 << 1;
pub const _ENetPacketFlag_ENET_PACKET_FLAG_NO_ALLOCATE: u32 = 1 << 2;
pub const _ENetPacketFlag_ENET_PACKET_FLAG_UNRELIABLE_FRAGMENTED: u32 = 1 << 3;
pub const _ENetPacketFlag_ENET_PACKET_FLAG_INSTANT: u32 = 1 << 4;
pub const _ENetPacketFlag_ENET_PACKET_FLAG_UNTHROTTLED: u32 = 1 << 5;
pub const _ENetPacketFlag_ENET_PACKET_FLAG_SENT: u32 = 1 << 8;

#[cfg(test)]
mod tests {
    use std::{ffi::CString, mem::MaybeUninit, time::Duration};

    use crate::*;

    #[test]
    fn server_test() {
        unsafe {
            if enet_initialize() != 0 {
                panic!("Enet could not be initialized.");
            }

            let address: MaybeUninit<ENetAddress> = MaybeUninit::uninit();
            let mut address = address.assume_init();
            // set the port to 8080
            address.port = 8080;
            // set the hostname to localhost 127.0.0.1; a CString must be used.
            let address_hostname = CString::new("127.0.0.1").unwrap();
            enet_address_set_hostname(&mut address, address_hostname.as_ptr());

            // create host accepting 8 clients max, with 2 different channels, and automatically setting
            // the in/out max bandwidth, and the buffer size by setting them to 0.
            let host = enet_host_create(&address, 8, 2, 0, 0, 0);

            // wait for a bit
            std::thread::sleep(Duration::from_millis(1000));

            // free the host and deinitialize ENet
            enet_host_destroy(host);
            enet_deinitialize();
        }
    }
}
