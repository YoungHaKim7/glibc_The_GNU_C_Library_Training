use libc::*;
use std::ffi::CString;
use std::mem;
use std::process;

const PORT: u16 = 5555;
const MESSAGE: &str = "Yow!!! Are we having fun yet?!?";
const SERVERHOST: &str = "127.0.0.1";

unsafe fn init_sockaddr(
    name: &mut sockaddr_in,
    hostname: &str,
    port: u16,
) {
    unsafe {
        let c_host = CString::new(hostname).unwrap();
        let c_port = format!("{}", port);
        let c_port = CString::new(c_port).unwrap();

        name.sin_family = AF_INET as u8;
        name.sin_port = htons(port);

        // Use getaddrinfo for address resolution (modern POSIX approach)
        let hints: addrinfo = mem::zeroed();
        let mut res: *mut addrinfo = std::ptr::null_mut();

        let status = getaddrinfo(
            c_host.as_ptr(),
            c_port.as_ptr(),
            &hints,
            &mut res,
        );

        if status != 0 {
            eprintln!("getaddrinfo: {}", hostname);
            process::exit(1);
        }

        if res.is_null() {
            eprintln!("No address found for {}", hostname);
            process::exit(1);
        }

        let addr = &mut *(*res).ai_addr as *mut sockaddr as *mut sockaddr_in;
        name.sin_addr = (*addr).sin_addr;

        freeaddrinfo(res);
    }
}

unsafe fn write_to_server(fd: i32) {
    unsafe {
        let msg = MESSAGE.as_bytes();

        let nbytes = write(
            fd,
            msg.as_ptr() as *const _,
            msg.len(),
        );

        if nbytes < 0 {
            perror(b"write\0".as_ptr() as *const _);
            process::exit(1);
        }
    }
}

fn main() {
    unsafe {
        let sock = socket(PF_INET, SOCK_STREAM, 0);

        if sock < 0 {
            perror(b"socket (client)\0".as_ptr() as *const _);
            process::exit(1);
        }

        let mut servername: sockaddr_in = mem::zeroed();

        init_sockaddr(
            &mut servername,
            SERVERHOST,
            PORT,
        );

        let result = connect(
            sock,
            &servername as *const _ as *const sockaddr,
            mem::size_of::<sockaddr_in>() as u32,
        );

        if result < 0 {
            perror(b"connect (client)\0".as_ptr() as *const _);
            process::exit(1);
        }

        write_to_server(sock);

        close(sock);
    }
}
