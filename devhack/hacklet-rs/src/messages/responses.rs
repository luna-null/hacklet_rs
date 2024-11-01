use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use nom::{
    self,
    Err, IResult, Needed,
};
use std::io::Write;
use super::Message;


#[derive(Debug)]
pub struct BootResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: Vec<u8>,
    pub device_id: u64,
    pub data2: u16,
    pub checksum: u8,
}
impl BootResponse
{
    pub fn new(data: Vec<u8>, device_id: u64, data2: u16) -> Self
    {
        let mut resp = BootResponse {
            header: 0x02,       // Default header
            command: 0x4084,    // Default command
            payload_length: 22, // TODO: payload length
            data,
            device_id,
            data2,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {

        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_all(&self.data).unwrap();
        buffer.write_u64::<BigEndian>(self.device_id).unwrap();
        buffer.write_u16::<BigEndian>(self.data2).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}

impl Message for BootResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_all(&self.data).unwrap();
        buffer.write_u64::<BigEndian>(self.device_id).unwrap();
        buffer.write_u16::<BigEndian>(self.data2).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }

    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 27 {
            return Err(Err::Incomplete(Needed::new(27)));
        }
        let data = input[4..16].to_vec();
        let device_id = u64::from_be_bytes(input[16..24].try_into().unwrap());
        let data2 = u16::from_be_bytes([input[24], input[25]]);
        let mut response = BootResponse::new(data, device_id, data2);
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.checksum = input[26];

        Ok((input, response))
    }
}

#[derive(Debug)]
pub struct BootConfirmResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: u8,
    pub checksum: u8,
}
impl BootConfirmResponse
{
    pub fn new() -> Self
    {
        let mut resp = BootConfirmResponse {
            header: 0x02,      // Default header
            command: 0x4080,   // Default command
            payload_length: 1, // TODO: payload length
            data: 0x10,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for BootConfirmResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 6 {
            return Err(Err::Incomplete(Needed::new(6)));
        }
        let mut response = BootConfirmResponse::new();
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.data = input[4];
        response.checksum = input[5];

        Ok((input, response))
    }
}

#[derive(Debug)]
pub struct BroadcastResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub network_id: u16,
    pub device_id: u64,
    pub data: u8,
    pub checksum: u8,
}
impl BroadcastResponse
{
    pub fn new(network_id: u16, device_id: u64, data: u8) -> Self
    {
        let mut resp = BroadcastResponse {
            header: 0x02,       // Default header
            command: 0xA013,    // Default command
            payload_length: 11, // TODO: payload length
            network_id,
            device_id,
            data,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u64::<BigEndian>(self.device_id).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for BroadcastResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u64::<BigEndian>(self.device_id).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 16 {
            return Err(Err::Incomplete(Needed::new(16)));
        }
        let network_id = u16::from_be_bytes([input[4], input[5]]);
        let device_id = u64::from_be_bytes(input[6..14].try_into().unwrap());
        let data = input[14];
        let mut response = BroadcastResponse::new(network_id, device_id, data);
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.checksum = input[15];

        Ok((input, response))
    }
}

#[derive(Debug)]
pub struct LockResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: u8,
    pub checksum: u8,
}
impl LockResponse
{
    pub fn new() -> Self
    {
        let mut resp = LockResponse {
            header: 0x02,       // Default header
            command: 0xA0F9,    // Default command
            payload_length: 1, // TODO: payload length
            data: 0x00,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for LockResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 6 {
            return Err(Err::Incomplete(Needed::new(6)));
        }
        let mut response = LockResponse::new();
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.data = input[4];
        response.checksum = input[5];

        Ok((
            input,
            response,
        ))
    }
}

#[derive(Debug)]
pub struct UpdateTimeAckResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: u8,
    pub checksum: u8,
}
impl UpdateTimeAckResponse
{
    pub fn new() -> Self
    {
        let mut resp = UpdateTimeAckResponse {
            header: 0x02,       // Default header
            command: 0x4022,    // Default command
            payload_length: 1, // TODO: payload length
            data: 0x00,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for UpdateTimeAckResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 6 {
            return Err(Err::Incomplete(Needed::new(6)));
        }
        let mut response = UpdateTimeAckResponse::new();
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.data = input[4];
        response.checksum = input[5];

        Ok((
            input,
            response,
        ))
    }
}

#[derive(Debug)]
pub struct UpdateTimeResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub network_id: u16,
    pub data: u8,
    pub checksum: u8,
}
impl UpdateTimeResponse
{
    pub fn new(network_id: u16) -> Self
    {
        let mut resp = UpdateTimeResponse {
            header: 0x02,       // Default header
            command: 0x40a2,    // Default command
            payload_length: 3, // TODO: payload length
            network_id,
            data: 0x00,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for UpdateTimeResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 8 {
            return Err(Err::Incomplete(Needed::new(8)));
        }
        let network_id = u16::from_be_bytes([input[4], input[5]]);
        let mut response = UpdateTimeResponse::new(network_id);
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.data = input[6];
        response.checksum = input[7];

        Ok((
            input,
            response,
        ))
    }
}

#[derive(Debug)]
pub struct HandshakeResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: u8,
    pub checksum: u8,
}
impl HandshakeResponse
{
    pub fn new() -> Self
    {
        let mut resp = HandshakeResponse {
            header: 0x02,       // Default header
            command: 0x4003,    // Default command
            payload_length: 1, // TODO: payload length
            data: 0x00,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for HandshakeResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 6 {
            return Err(Err::Incomplete(Needed::new(6)));
        }
        let mut response = HandshakeResponse::new();
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.data = input[4];
        response.checksum = input[5];

        Ok((
            input,
            response,
        ))
    }
}

#[derive(Debug)]
pub struct AckResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: u8,
    pub checksum: u8,
}
impl AckResponse
{
    pub fn new() -> Self
    {
        let mut resp = AckResponse {
            header: 0x02,       // Default header
            command: 0x4024,    // Default command
            payload_length: 1, // TODO: payload length
            data: 0x00,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for AckResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 6 {
            return Err(Err::Incomplete(Needed::new(6)));
        }
        let mut response = AckResponse::new();
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.data = input[4];
        response.checksum = input[5];

        Ok((
            input,
            response,
        ))
    }
}

#[derive(Debug)]
pub struct SamplesResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub network_id: u16,
    pub channel_id: u16,
    pub data: u16,
    pub time: u32, // little-endian
    pub sample_count: u8,
    pub stored_sample_count: u32, // 3-byte, little-endian
    pub samples: Vec<u16>,
    pub checksum: u8,
}
impl SamplesResponse {
    pub fn new(
        payload_length: u8, 
        network_id: u16, 
        channel_id: u16, 
        data: u16, 
        sample_count: u8, 
        time: u32, 
        stored_sample_count : u32, 
        samples: Vec<u16>
    ) -> Self {
        let mut req = SamplesResponse {
            header: 0x02,           // Default header
            command: 0x40A4,        // Default command
            payload_length,      // TODO: payload length
            network_id,
            channel_id,
            data,
            time,
            sample_count,
            stored_sample_count,
            samples,
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
        buffer.write_u16::<BigEndian>(self.channel_id).unwrap();
        buffer.write_u16::<BigEndian>(self.data).unwrap();
        buffer.write_u32::<LittleEndian>(self.time).unwrap();
        buffer.write_u8(self.sample_count).unwrap();
        buffer
            .write_u8((self.stored_sample_count & 0xFF) as u8)
            .unwrap();
        buffer
            .write_u8(((self.stored_sample_count >> 8) & 0xFF) as u8)
            .unwrap();
        buffer
            .write_u8(((self.stored_sample_count >> 16) & 0xFF) as u8)
            .unwrap();
        for sample in &self.samples {
            buffer.write_u16::<LittleEndian>(*sample).unwrap();
        }
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for SamplesResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u16::<BigEndian>(self.network_id).unwrap();
        buffer.write_u16::<BigEndian>(self.channel_id).unwrap();
        buffer.write_u16::<BigEndian>(self.data).unwrap();
        buffer.write_u32::<LittleEndian>(self.time).unwrap();
        buffer.write_u8(self.sample_count).unwrap();
        buffer
            .write_u8((self.stored_sample_count & 0xFF) as u8)
            .unwrap();
        buffer
            .write_u8(((self.stored_sample_count >> 8) & 0xFF) as u8)
            .unwrap();
        buffer
            .write_u8(((self.stored_sample_count >> 16) & 0xFF) as u8)
            .unwrap();
        for sample in &self.samples {
            buffer.write_u16::<LittleEndian>(*sample).unwrap();
        }
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 18 {
            return Err(Err::Incomplete(Needed::new(18)));
        }
        let payload_length = input[3];
        let network_id = u16::from_be_bytes([input[4], input[5]]);
        let channel_id = u16::from_be_bytes([input[6], input[7]]);
        let data = u16::from_be_bytes([input[8], input[9]]);
        let time = u32::from_le_bytes([input[10], input[11], input[12], input[13]]);
        let sample_count = input[14];
        let stored_sample_count = u32::from_le_bytes([0, input[15], input[16], input[17]]);
        let mut samples = Vec::new();
        let sample_bytes_start = 18;
        let sample_bytes_end = sample_bytes_start + (sample_count as usize) * 2;
        if input.len() < sample_bytes_end + 1 {
            return Err(Err::Incomplete(Needed::new(sample_bytes_end + 1)));
        }
        for i in (sample_bytes_start..sample_bytes_end).step_by(2) {
            let sample = u16::from_le_bytes([input[i], input[i + 1]]);
            samples.push(sample);
        }
        let mut response = SamplesResponse::new(payload_length, network_id, channel_id, data, sample_count, time, stored_sample_count, samples);
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.checksum = input[sample_bytes_end];

        Ok((
            input,
            response,
        ))
    }
}

#[derive(Debug)]
pub struct ScheduleResponse
{
    pub header: u8,
    pub command: u16,
    pub payload_length: u8,
    pub data: u8,
    pub checksum: u8,
}
impl ScheduleResponse
{
    pub fn new() -> Self
    {
        let mut resp = ScheduleResponse {
            header: 0x02,      // Default header
            command: 0x4023,   // Default command
            payload_length: 1, 
            data: 0x00,
            checksum: 0,
        };
        resp.checksum = resp.calculate_checksum();
        resp
    }
    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut buffer = Vec::new();
        buffer.write_u8(self.header).unwrap();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.write_u8(self.checksum).unwrap();
        buffer
    }
}
impl Message for ScheduleResponse
{
    fn calculate_checksum(&self) -> u8
    {
        let mut buffer = Vec::new();
        buffer.write_u16::<BigEndian>(self.command).unwrap();
        buffer.write_u8(self.payload_length).unwrap();
        buffer.write_u8(self.data).unwrap();
        buffer.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    {
        if input.is_empty() || input.len() < 6 {
            return Err(Err::Incomplete(Needed::new(6)));
        }
        let mut response = ScheduleResponse::new();
        response.header = input[0];
        response.command = u16::from_be_bytes([input[1], input[2]]);
        response.payload_length = input[3];
        response.data = input[4];
        response.checksum = input[5];

        Ok((
            input,
            response,
        ))
    }
}

