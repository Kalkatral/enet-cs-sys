use std::{ffi::CString, mem::MaybeUninit, ptr::null};

use enet_cs_sys::*;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;
const CONNECTION_TIMEOUT: u32 = 5000;
const DURATION: u32 = 10;
const MESSAGE: &str = "Sussy amogus";
const MESSAGE_CHANNEL: u8 = 0;

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
        panic!("Invalid hostname \"{}\"", ADDRESS);
    }

    let host = unsafe { enet_host_create(null(), 1, 2, 0, 0, 0) };

    if host.is_null() {
        panic!("Could not create host.");
    }

    println!("Host created.");

    let server_peer = unsafe { enet_host_connect(host, &address, 2, 0) };

    println!("Waiting for server to accept connection.");

    let mut event = unsafe { MaybeUninit::zeroed().assume_init() };

    if unsafe { enet_host_service(host, &mut event, CONNECTION_TIMEOUT) } > 0 {
        #[allow(non_upper_case_globals)]
        match event.type_ {
            _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                println!(
                    "Connection to server succeeded. Sending messages for {} seconds.",
                    DURATION
                );
                assert_eq!(server_peer as usize, event.peer as usize);
            }
            _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                panic!("Server denied connection.");
            }
            _ENetEventType_ENET_EVENT_TYPE_DISCONNECT_TIMEOUT => {
                panic!("Server connection timed out.");
            }
            _ => panic!("wait what"),
        }
    } else {
        panic!("Could not connect to server.");
    }

    for _i in 0..DURATION {
        println!(
            "Sending \"{}\" to server on channel {}.",
            MESSAGE, MESSAGE_CHANNEL
        );

        let packet = unsafe {
            enet_packet_create(
                MESSAGE.as_ptr().cast(),
                MESSAGE.len(),
                _ENetPacketFlag_ENET_PACKET_FLAG_RELIABLE,
            )
        };

        unsafe {
            enet_peer_send(server_peer, MESSAGE_CHANNEL, packet);
        }

        // do not destroy the packet, after calling enet_peer_send, destruction is handled by ENet

        if unsafe { enet_host_service(host, &mut event, 1000) } > 0 {
            #[allow(non_upper_case_globals)]
            match event.type_ {
                _ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                    println!("Received connection event for some unholy reason.");
                }
                _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                    println!("Disconnected from server.");
                }
                _ENetEventType_ENET_EVENT_TYPE_DISCONNECT_TIMEOUT => {
                    println!("Connection to server timed out.");
                }
                _ENetEventType_ENET_EVENT_TYPE_RECEIVE => {
                    println!(
                        "Received message from server. (packet size: {}B)",
                        unsafe { *event.packet }.dataLength
                    );
                }
                _ENetEventType_ENET_EVENT_TYPE_NONE => {}
                _ => unreachable!(),
            }
        }
    }

    println!("Disconnecting.");

    unsafe {
        enet_peer_disconnect_later(server_peer, 0);
    }

    if unsafe { enet_host_service(host, &mut event, CONNECTION_TIMEOUT) } > 0 {
        #[allow(non_upper_case_globals)]
        match event.type_ {
            _ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                println!("Disconnection succesful.");
            }
            _ENetEventType_ENET_EVENT_TYPE_DISCONNECT_TIMEOUT => {
                println!("Server connection timed out.");
            }
            _ => println!("wait what"),
        }
    } else {
        println!("Could not disconnect gracefully, forcefully disconnecting.");

        unsafe {
            enet_peer_reset(server_peer);
        }
    }

    unsafe {
        enet_host_destroy(host);
        enet_deinitialize();
    }
}
