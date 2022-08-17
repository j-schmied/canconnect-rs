//!
//! Library for CAN Connections and Security Testing
//! (c) Jannik Schmied, 2022
//!
//! Credits: socketcan-rs
//!

mod canconnection;
mod mitmfilter;

#[cfg(test)]
mod test {
    use crate::*;
    use crate::canconnection::CANConnection;

    #[test]
    fn it_works() {
        let interface = String::from("vcan0");
        let vcan_if = canconnection::CANConnection::new(&interface);

        let data: [u8;4] = [0x0; 4];
        let mut frame = CANConnection::create_can_frame(0x300, &data, false, false);

        if vcan_if.can_recv(&mut frame) {
            !dbg!(frame);
        }
    }
}