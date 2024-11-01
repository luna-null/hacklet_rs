use std::time::{Duration, Instant};

use log::{self, info};

use crate::{
    messages::{requests::*, responses::*, Message},
    serial_connection::*,
};

pub struct Dongle {
    serial: SerialConnection,
}

impl Dongle {
    // Open method - Initializes and yields a dongle instance
    pub fn open<F>(callback: F)
    where
        F: FnOnce(&mut Dongle) -> ()
    {
        let serial = SerialConnection::new();
        let mut dongle = Dongle { serial };

        dongle.boot();
        dongle.boot_confirm();
        callback(&mut dongle);

        // Serial connection is closed at the end (Drop implemented in Rust can handle this)
    }

    pub fn new(serial: SerialConnection) -> Self {
        Dongle { serial }
    }

    // Commission method - listens for new devices on the network
    pub fn commission(&mut self) {
        let mut response: Option<BroadcastResponse> = None;
        self.unlock_network();

        let timeout = Duration::from_secs(30);
        let start_time = Instant::now();

        while start_time.elapsed() < timeout {
            info!("Listening for devices ...");
            let mut buffer = self.serial.receive(4);
            buffer.extend(self.serial.receive(buffer[3] as usize + 1));

            if buffer[1] == 0xa0 {
                response = Some(BroadcastResponse::read(&buffer).unwrap().1);
                let resp = response.as_ref().unwrap();
                info!("{}",
                    &format!("Found device 0x{:x} on network 0x{:x}", resp.device_id, resp.network_id)
                );
                break;
            }
        }

        if let Some(resp) = response {
            self.update_time(resp.network_id);
        }
        self.lock_network();
    }

    // Selects the network
    pub fn select_network(&mut self, network_id: u16) {
        self.serial.transmit(&HandshakeRequest::new(network_id).as_bytes());
        let _ = HandshakeResponse::read(&self.serial.receive(6));
    }

    // Request samples
    pub fn request_samples(&mut self, network_id: u16, channel_id: u16) -> SamplesResponse {
        info!("Requesting samples");
        self.serial.transmit(&SamplesRequest::new(network_id, channel_id).as_bytes());
        let _ = AckResponse::read(&self.serial.receive(6));

        let mut buffer = self.serial.receive(4);
        let remaining_bytes = buffer[3] as usize + 1;
        buffer.extend(self.serial.receive(remaining_bytes));

        let response = SamplesResponse::read(&buffer).unwrap().1;

        for sample in response.samples.iter() {
            let (time, wattage) = ((*sample >> 8) as u8, (*sample & 0xFF) as u8);
            info!("{}", format!("{}w at {}", wattage, time));
        }

        info!("{}", format!(
            "{} returned, {} remaining",
            response.sample_count, response.stored_sample_count
        ));

        response
    }

    // Switch a socket on or off
    pub fn switch(&mut self, network_id: u16, channel_id: u16, state: bool) {
        let mut request = ScheduleRequest::new(network_id, channel_id);

        if state {
            request.always_on();
            info!("{}", format!(
                "Turning on channel {} on network 0x{:x}", channel_id, network_id
            ));
        } else {
            request.always_off();
            info!("{}", format!(
                "Turning off channel {} on network 0x{:x}", channel_id, network_id
            ));
        }

        self.serial.transmit(&request.as_bytes());
        let _ = ScheduleResponse::read(&self.serial.receive(6));
    }

    // Unlock the network
    pub fn unlock_network(&mut self) {
        info!("Unlocking network");
        self.serial.transmit(&UnlockRequest::new().as_bytes());
        let _ = LockResponse::read(&self.serial.receive(6));
        info!("Unlocking complete");
    }

    // Lock the network
    pub fn lock_network(&mut self) {
        info!("Locking network");
        self.serial.transmit(&LockRequest::new().as_bytes());
        let _ = LockResponse::read(&self.serial.receive(6));
        info!("Locking complete");
    }

    // Boot the dongle
    fn boot(&mut self) {
        info!("Booting");
        self.serial.transmit(&BootRequest::new().as_bytes());
        let _ = BootResponse::read(&self.serial.receive(27));
    }

    // Confirm boot success
    fn boot_confirm(&mut self) {
        self.serial.transmit(&BootConfirmRequest::new().as_bytes());
        let _ = BootConfirmResponse::read(&self.serial.receive(6));
        info!("Booting complete");
    }

    // Update device time
    fn update_time(&mut self, network_id: u16) {
        self.serial.transmit(&UpdateTimeRequest::new(network_id).as_bytes());
        let _ = UpdateTimeAckResponse::read(&self.serial.receive(6));
        let _ = UpdateTimeResponse::read(&self.serial.receive(8));
    }
}

#[cfg(test)]
mod tests {
    use mockall::{automock, predicate::*};

    #[automock]
    pub trait SerialPortTrait {
        fn write_data(&mut self, data: &[u8]) -> Result<usize, std::io::Error>;
        fn read_data(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;
        fn usb_close(&mut self);
    }

    pub struct HackletDongle<P: SerialPortTrait> {
        serial_port: P,
    }

    impl<P: SerialPortTrait> HackletDongle<P> {
        pub fn open(serial_port: P) -> Self {
            HackletDongle { serial_port }
        }

        pub async fn boot(&mut self) -> Result<(), std::io::Error> {
            self.serial_port.write_data(&[0x02, 0x40, 0x04, 0x00, 0x44])?;
            let mut buffer = [0u8; 32];
            self.serial_port.read_data(&mut buffer)?;
            // Process response here...

            Ok(())
        }

        pub async fn request_sample(&mut self) -> Result<(), std::io::Error> {
            // Implementation of request_sample...
            Ok(())
        }

        pub fn close(&mut self) {
            self.serial_port.usb_close();
        }
    }

    #[tokio_macros::test]
    async fn can_open_new_session() {
        let mut mock_serial = MockSerialPortTrait::new();

        // Expectations for boot sequence
        mock_serial.expect_write_data()
            .with(eq([0x02, 0x40, 0x04, 0x00, 0x44].as_ref()))
            .times(1)
            .returning(|_| Ok(5));

        mock_serial.expect_read_data()
            .with(eq([0u8; 32].as_mut()))
            .times(1)
            .returning(|buf| {
                buf.copy_from_slice(&[0x02, 0x40, 0x84, 0x16, 0x01, 0x00, 0x00, 0x87]);
                Ok(8)
            });

        mock_serial.expect_usb_close()
            .times(1);

        let mut dongle = HackletDongle::open(mock_serial);
        dongle.boot().await.unwrap();
        dongle.close();
    }

    #[tokio_macros::test]
    async fn can_find_new_device() {
        let mut mock_serial = MockSerialPortTrait::new();

        // Expectations for finding a new device
        mock_serial.expect_write_data()
            .with(eq([0x02, 0x40, 0x04, 0x00, 0x44].as_ref()))
            .times(1)
            .returning(|_| Ok(5));

        mock_serial.expect_read_data()
            .with(eq([0u8; 32].as_mut()))
            .times(1)
            .returning(|buf| {
                buf.copy_from_slice(&[0x02, 0x40, 0x84, 0x16, 0x01, 0x00, 0x00, 0x87]);
                Ok(8)
            });

        mock_serial.expect_usb_close()
            .times(1);

        let mut dongle = HackletDongle::open(mock_serial);
        dongle.boot().await.unwrap();
        dongle.close();
    }

    #[tokio_macros::test]
    async fn can_request_sample() {
        let mut mock_serial = MockSerialPortTrait::new();

        // Expectations for requesting a sample
        mock_serial.expect_write_data()
            .with(eq([0x02, 0x40, 0x04, 0x00, 0x44].as_ref()))
            .times(1)
            .returning(|_| Ok(5));

        mock_serial.expect_read_data()
            .with(eq([0u8; 32].as_mut()))
            .times(1)
            .returning(|buf| {
                buf.copy_from_slice(&[0x02, 0x40, 0x84, 0x16, 0x01, 0x00, 0x00, 0x87]);
                Ok(8)
            });

        // Additional expectations for requesting sample...
        mock_serial.expect_usb_close()
            .times(1);

        let mut dongle = HackletDongle::open(mock_serial);
        dongle.request_sample().await.unwrap();
        dongle.close();
    }

    #[tokio_macros::test]
    async fn can_enable_socket() {
        let mut mock_serial = MockSerialPortTrait::new();

        // Expectations for enabling socket
        mock_serial.expect_write_data()
            .with(eq([0x02, 0x40, 0x04, 0x00, 0x44].as_ref()))
            .times(1)
            .returning(|_| Ok(5));

        mock_serial.exp
    }
}
