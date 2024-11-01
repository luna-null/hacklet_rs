use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use nom::{
    self,
    Err, IResult, Needed,
};
use std::{
    io::Write, time::{SystemTime, UNIX_EPOCH}
};
use super::Message;

#[derive(Debug)]
pub struct BootRequest
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub checksum: u8,
}
impl BootRequest
{
    pub fn new() -> Self
    {
        let mut req = BootRequest {
            header: 0x02,      // Default header
            command: 0x4004,   // Default command
            payload_length: 0, // TODO: payload length
            checksum: 0,
        };
        req.checksum = req.calculate_checksum();
        req
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for BootRequest
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 5 {
            return Err(Err::Incomplete(Needed::new(5)));
        }
        let mut request = BootRequest::new();
        request.header = input[0];
        request.command = u16::from_be_bytes([input[1], input[2]]);
        request.payload_length = input[3];
        request.checksum = input[4];

        Ok((input, request))
    }
}

#[derive(Debug)]
pub struct BootConfirmRequest
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub checksum: u8,
}
impl BootConfirmRequest
{
    pub fn new() -> Self
    {
        let mut req = BootConfirmRequest {
            header: 0x02,    // Default header
            command: 0x4000, // Default command
            payload_length: 1,
            checksum: 0,
        };
        req.checksum = req.calculate_checksum();
        req
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for BootConfirmRequest
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 5 {
            return Err(Err::Incomplete(Needed::new(5)));
        }
        let mut request = BootConfirmRequest::new();
        request.header = input[0];
        request.command = u16::from_be_bytes([input[1], input[2]]);
        request.payload_length = input[3];
        request.checksum = input[4];

        Ok((input, request))
    }
}

#[derive(Debug)]
pub struct UnlockRequest
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: u32,
    pub checksum: u8,
}
impl UnlockRequest
{
    pub fn new() -> Self
    {
        let mut req = UnlockRequest {
            header: 0x02,      // Default header
            command: 0xA236,   // Default command
            payload_length: 4, // Default payload length
            data: 0xFCFF9001,  // Default time
            checksum: 0,
        };
        req.checksum = req.calculate_checksum();
        req
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u32::<BigEndian>(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for UnlockRequest
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u32::<BigEndian>(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 9 {
            return Err(Err::Incomplete(Needed::new(9)));
        }
        let mut request = UnlockRequest::new();
        request.header = input[0];
        request.command = u16::from_be_bytes([input[1], input[2]]);
        request.payload_length = input[3];
        request.data = u32::from_be_bytes(input[4..8].try_into().unwrap());
        request.checksum = input[8];

        Ok((input, request))
    }
}

#[derive(Debug)]
pub struct LockRequest
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: u32,
    pub checksum: u8,
}
impl LockRequest
{
    pub fn new() -> Self
    {
        let mut req = LockRequest {
            header: 0x02,      // Default header
            command: 0xA236,   // Default command
            payload_length: 4, // Default payload length
            data: 0xFCFF0001,  // Default time
            checksum: 0,       // TODO: calculate_checksum
        };
        req.checksum = req.calculate_checksum();
        req
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u32::<BigEndian>(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for LockRequest
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u32::<BigEndian>(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 9 {
            return Err(Err::Incomplete(Needed::new(9)));
        }
        let mut request = LockRequest::new();
        request.header = input[0];
        request.command = u16::from_be_bytes([input[1], input[2]]);
        request.payload_length = input[3];
        request.data = u32::from_be_bytes(input[4..8].try_into().unwrap());
        request.checksum = input[8];

        Ok((input, request))
    }
}

#[derive(Debug)]
pub struct UpdateTimeRequest
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub network_id: u16,
    pub time: u32,
    pub checksum: u8,
}
impl UpdateTimeRequest
{
    pub fn new(network_id: u16) -> Self
    {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32;
        let mut req = UpdateTimeRequest {
            header: 0x02,      // Default header
            command: 0x4022,   // Default command
            payload_length: 6, // Default payload length
            network_id,
            time,        
            checksum: 0, 
        };
        req.checksum = req.calculate_checksum();
        req
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u32::<LittleEndian>(self.time).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for UpdateTimeRequest
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u32::<LittleEndian>(self.time).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 11 {
            return Err(Err::Incomplete(Needed::new(11)));
        }
        let network_id = u16::from_be_bytes([input[4], input[5]]);
        let mut request = UpdateTimeRequest::new(network_id);
        request.header = input[0];
        request.command = u16::from_be_bytes([input[1], input[2]]);
        request.payload_length = input[3];
        request.time = u32::from_le_bytes(input[6..10].try_into().unwrap());
        request.checksum = input[10];

        Ok((input, request))
    }
}

#[derive(Debug)]
pub struct HandshakeRequest
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub network_id: u16,
    pub data: u16,
    pub checksum: u8,
}
impl HandshakeRequest
{
    pub fn new(network_id: u16) -> Self
    {
        let mut req = HandshakeRequest {
            header: 0x02,      // Default header
            command: 0x4003,   // Default command
            payload_length: 4, // Default payload length
            network_id,        // TODO: network_id func
            data: 0x0500,      // Default time
            checksum: 0,       // TODO: calculate_checksum
        };
        req.checksum = req.calculate_checksum(); // Set checksum based on the other fields
        req
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u16::<BigEndian>(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for HandshakeRequest
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u16::<BigEndian>(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 9 {
            return Err(Err::Incomplete(Needed::new(9)));
        }
        let network_id = u16::from_be_bytes([input[4], input[5]]);
        let mut request = HandshakeRequest::new(network_id);
        request.header = input[0];
        request.command = u16::from_be_bytes([input[1], input[2]]);
        request.payload_length = input[3];
        request.data = u16::from_be_bytes([input[6], input[7]]);
        request.checksum = input[8];

        Ok((input, request))
    }
}

#[derive(Debug)]
pub struct SamplesRequest
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub network_id: u16,
    pub channel_id: u16,
    pub data: u16,
    pub checksum: u8,
}
impl SamplesRequest
{
    pub fn new(network_id: u16, channel_id: u16) -> Self
    {
        let mut req = SamplesRequest {
            header: 0x02,      // Default header
            command: 0x4024,   // Default command
            payload_length: 6, // Default payload length
            network_id,   // TODO: network_id
            channel_id,   // TODO: channel_id
            data: 0x0A00,
            checksum: 0, // TODO: calculate_checksum
        };
        req.checksum = req.calculate_checksum(); // Set checksum based on the other fields
        req
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u16::<BigEndian>(self.channel_id).unwrap();
        buffer.write_u16::<BigEndian>(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}

impl Message for SamplesRequest
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u16::<BigEndian>(self.channel_id).unwrap();
        buffer.write_u16::<BigEndian>(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }

    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 10 {
            return Err(Err::Incomplete(Needed::new(27)));
        }
        let network_id = u16::from_be_bytes([input[3], input[4]]);
        let channel_id = u16::from_be_bytes([input[5], input[6]]);
        let mut request = SamplesRequest::new(network_id, channel_id);
        request.header = input[0];
        request.command = u16::from_be_bytes([input[1], input[2]]);
        request.payload_length = input[3];
        request.data = u16::from_be_bytes([input[7], input[8]]);
        request.checksum = input[9];

        Ok((input, request))
    }
}

#[derive(Debug)]
pub struct ScheduleRequest
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub network_id: u16,
    pub channel_id: u16,
    pub schedule: Vec<u8>,
    pub checksum: u8,
}
impl ScheduleRequest
{
    pub fn new(network_id: u16, channel_id: u16) -> Self
    {
        let mut req = ScheduleRequest {
            header: 0x02,             // Default header
            command: 0x4023,          // Default command
            payload_length: 59,       // Default payload length
            network_id,          // TODO: network_id
            channel_id,          // TODO: channel_id
            schedule: vec![0x00, 56], // Default time
            checksum: 0,              // TODO: calculate_checksum
        };
        req.checksum = req.calculate_checksum(); // Set checksum based on the other fields
        req
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u16::<BigEndian>(self.channel_id).unwrap();
        buffer.write_all(&self.schedule).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
    pub fn always_on(&mut self) 
    {
        let mut bitmap = vec![0x7f, 56];
        bitmap[5] = 0x25;
        self.schedule = bitmap;

    }
    pub fn always_off(&mut self) 
    {
        let mut bitmap = vec![0x7f, 56];
        bitmap[5] = 0xa5;
        self.schedule = bitmap;

    }
}
impl Message for ScheduleRequest
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u16::<BigEndian>(self.channel_id).unwrap();
        buffer.write_all(&self.schedule).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 65 {
            return Err(Err::Incomplete(Needed::new(65)));
        }
        let network_id = u16::from_be_bytes([input[4], input[5]]);
        let channel_id = u16::from_be_bytes([input[6], input[7]]);
        let mut request = ScheduleRequest::new(network_id, channel_id);
        request.header = input[0];
        request.command = u16::from_be_bytes([input[1], input[2]]);
        request.payload_length = input[3];
        request.schedule = input[8..64].to_vec();
        request.checksum = input[64];

        Ok((input, request))
    }
}
