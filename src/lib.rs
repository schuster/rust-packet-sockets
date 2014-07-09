extern crate libc;

use libc::{c_char, c_int, c_ushort, c_uint, c_void};
use std::c_str::CString;

extern {
    fn printf(s: *const libc::c_char) -> libc::c_int;
}

extern {
    // takes a pointer to the first item in an array of ifaddr pointers (in
    // other words, a pointer to a pointer to an ifaddr)
    fn getifaddrs(addrs: *mut *mut ifaddrs) -> c_int;
}

// TODO: try to call getifaddrs (and freeifaddrs, too) to get the list of interfaces, just like in Tony's code

struct sockaddr {
  sa_family: c_ushort,
  sa_data: [c_char, ..14]
}

struct ifaddrs {
    ifa_next: *const ifaddrs,
    ifa_name: *const c_char,
    ifa_flags: c_uint,
    ifa_addr: *const sockaddr,
    ifa_netmask: *const sockaddr,
    ifa_ifu: *const sockaddr,
    ifa_data: *const c_void
}

fn main() {
    let x =
        unsafe {
            printf("Hello, world!\n".to_c_str().as_ptr())
        };

    println!("{}", x);

    // let addrs = ifaddrs {
    //     ifa_next : std::ptr::mut_null(),
    //     ifa_name : std::ptr::mut_null(),
    //     ifa_flags : 0,
    //     ifa_addr : std::ptr::mut_null(),
    //     ifa_netmask : std::ptr::mut_null(),
    //     ifa_ifu : std::ptr::mut_null(),
    //     ifa_data : std::ptr::mut_null(),
    // };
    // TOOD: figure out a better default capacity than 20 (see the ptr module: I think I just need to give it a mutable pointer; no allocation needed)
    let mut addrs : Vec<*mut ifaddrs> = Vec::with_capacity(1);

    unsafe {
        let ptr = addrs.as_mut_ptr(); // *mut *mut ifaddr
        let ret = getifaddrs(ptr);
        println!("return code: {}", ret);

        let first_ptr = *ptr;
        println!("{}", first_ptr);
        let first_addr = *first_ptr;
        println!("{}", first_addr.ifa_next);
        println!("{}", CString::new(first_addr.ifa_name, false).as_str());



        // let first_ptr = addrs.get(0);
        // let first_addr = *first_ptr;
        // let first_addr_addr = *first_addr;
        // first_addr_addr.ifa_flags;
        // println!("{}", first_addr_addr.ifa_name);
    }

}
