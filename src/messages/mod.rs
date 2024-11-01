use nom::{
    self,
    IResult,
};

pub mod responses;
pub mod requests;

pub trait Message
{
    fn calculate_checksum(&self) -> u8;
    fn read(bytes: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use crate::messages::responses::BootConfirmResponse;
    use crate::messages::requests::BootRequest;

    #[test]
    fn boot_confirm_response_detects_invalid_checksum() {
        let bad_checksum = vec![0x02, 0x40, 0x80, 0x01, 0x10, 0x01];

        // Expect an error when reading the invalid checksum
        let result = BootConfirmResponse::read(&bad_checksum);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid checksum");
    }

    #[test]
    fn boot_request_has_proper_checksum() {
        let request = BootRequest::new();
        assert_eq!(request.checksum().get(), 0x44); // Check the checksum
    }
}
