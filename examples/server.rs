use std::{ffi::CString, mem::MaybeUninit};

use enet_cs_sys::*;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;
const DURATION: u32 = 30;

fn main() {
    if unsafe { enet_initialize() } != 0 {
        panic!("Could not initialize ENet.");
    }

    println!("ENet initialized.");

    let address: MaybeUninit<ENetAddress> = MaybeUninit::uninit();
    let mut address = unsafe { address.assume_init() };

    address.port = PORT;

    let address_hostname = CString::new(ADDRESS).unwrap();

    if unsafe { enet_address_set_hostname(&mut address, address_hostname.as_ptr()) } != 0 {
        panic!("Invalid hostname \"{}\".", ADDRESS);
    }

    let host = unsafe { enet_host_create(&address, 8, 2, 0, 0, 0) };

    if host.is_null() {
        panic!("Failed to create host.");
    }

    println!(
        "Host created. Polling events for the next {} seconds.",
        DURATION
    );

    let mut event = unsafe { MaybeUninit::zeroed().assume_init() };

    for _i in 0..DURATION {
        if unsafe { enet_host_service(host, &mut event, 1000) } > 0 {
            #[allow(non_upper_case_globals)]
            match event.type_ {
                _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                    println!(
                        "A peer connected. (id: {})",
                        unsafe { *event.peer }.connectID
                    );
                }
                _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                    println!("A peer disconnected.");
                }
                _ENetEventType_ENET_EVENT_TYPE_DISCONNECT_TIMEOUT => {
                    println!("A peer timed out.");
                }
                _ENetEventType_ENET_EVENT_TYPE_RECEIVE => {
                    println!(
                        "Received a packet. (sender peer id: {}, packet size: {}B)",
                        unsafe { *event.peer }.connectID,
                        unsafe { *event.packet }.dataLength
                    );
                    unsafe { enet_packet_destroy(event.packet) };
                }
                _ENetEventType_ENET_EVENT_TYPE_NONE => {}
                _ => unreachable!(),
            }
        }
    }

    unsafe {
        enet_host_destroy(host);
        enet_deinitialize();
    }
}
