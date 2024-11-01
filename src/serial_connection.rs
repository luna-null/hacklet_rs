use log::{debug, info};
use libftdi1_sys::*;
use std::{
    thread::sleep,
    time::Duration,
};

use crate::messages::requests::BootRequest;

pub const SIO_DISABLE_FLOW_CTRL: u32 = 0;

#[derive(Debug)]
pub struct SerialConnection {
    context: *mut libftdi1_sys::ftdi_context,
    receive_buffer: Vec<u8>,
}

impl SerialConnection {
    pub fn new() -> Self
    {
        let context = unsafe { ftdi_new() };

        unsafe {
            if ftdi_usb_open(context, 0x0403, 0x8c81) < 0 {
                panic!("Could not open FTDI device");
            }

            // Set bitmode and baudrate
            ftdi_set_bitmode(context, 0x00, ftdi_mpsse_mode::BITMODE_RESET.0 as u8);
            ftdi_set_baudrate(context, 115200);
            ftdi_setflowctrl(context, SIO_DISABLE_FLOW_CTRL as i32);
            ftdi_setdtr(context, 1);
            ftdi_setrts(context, 1);
        }

        SerialConnection
        {
            context,
            receive_buffer: vec![],
        }

    }

    pub fn close(&mut self) 
    {
        unsafe {
            ftdi_usb_close(self.context);
            ftdi_free(self.context);
        }
        info!("Closed FTDI device connection");
    }

    pub fn transmit(&mut self, command: &[u8]) 
    {
        debug!("TX: {:?}", command);
        unsafe {
            if ftdi_write_data(self.context, command.as_ptr(), command.len() as i32) < 0 {
                panic!("Failed to write data");
            }
        }
    }

    pub fn receive(&mut self, bytes: usize) -> Vec<u8>
    {
        loop {
            if self.receive_buffer.len() >= bytes {
                let response: Vec<u8> = self.receive_buffer.drain(..bytes).collect();
                debug!("RX: {:?}", response);
                return response;
            }

            let mut buf = [0u8; 64]; // Buffer for reading data
            unsafe {
                let chunk = ftdi_read_data(self.context, buf.as_mut_ptr(), buf.len() as i32);
                if chunk > 0 {
                    self.receive_buffer.extend_from_slice(&buf[..chunk as usize]);
                } else {
                    sleep(Duration::from_millis(100));
                }
            }
        }
    }
}
// pub fn unpack(message: &[u8]) -> Vec<String>
// {
//     message.iter().map(|byte| format!("{:0.2x}", byte)).collect()
// }


#[derive(Debug)]
pub enum BitbangMode {
    Reset,
}



#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::{mock};
    use libftdi1_sys;

    mock! {
        pub FtdiContext {}
        impl FtdiContext for FtdiContext {
            fn usb_open(&self, vendor_id: u16, product_id: u16);
            fn set_bitmode(&self, bitmask: u8, mode: BitbangMode);
            fn set_baudrate(&self, baudrate: u32);
            fn set_flowctrl(&self, flowctrl: u32);
            fn set_dtr(&self, value: u8);
            fn set_rts(&self, value: u8);
            fn write_data(&self, data: &[u8]);
            fn read_data(&self) -> Vec<u8>;
        }
    }

    #[test]
    fn initialization_creates_serial_port_with_no_flow_control() {
        let mut ftdi = MockFtdiContext::new();
        ftdi.expect_usb_open()
            .with(eq(0x0403), eq(0x8c81))
            .times(1);
        ftdi.expect_set_bitmode()
            .with(eq(0x00), eq(BitbangMode::Reset))
            .times(1);
        ftdi.expect_set_baudrate()
            .with(eq(115200))
            .times(1);
        ftdi.expect_set_flowctrl()
            .with(eq(SIO_DISABLE_FLOW_CTRL))
            .times(1);
        ftdi.expect_set_dtr()
            .with(eq(1))
            .times(1);
        ftdi.expect_set_rts()
            .with(eq(1))
            .times(1);

        SerialConnection::new(Box::new(ftdi));
    }

    #[test]
    fn transmitting_is_successful() {

        let mut ftdi = MockFtdiContext::new();
        ftdi.expect_write_data()
            .with(eq(vec![0x02, 0x40, 0x04, 0x00, 0x44]))
            .times(1);

        let mut connection = SerialConnection::new(Box::new(ftdi));
        connection.transmit(&BootRequest::new().as_bytes());
    }

    #[test]
    fn receiving_is_successful() {

        let mut ftdi = MockFtdiContext::new();
        ftdi.expect_read_data()
            .returning(|| vec![0x02]);

        let mut connection = SerialConnection::new(Box::new(ftdi));
        let data = connection.receive(1);
        assert_eq!(data, vec![0x02]);
    }
}
