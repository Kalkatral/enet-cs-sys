#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

include! {concat!(env!("OUT_DIR"), "/bindings.rs")}

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
