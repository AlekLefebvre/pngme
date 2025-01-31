use std::{fmt, string::FromUtf8Error};

use crate::chunk_type::ChunkType;

pub(crate) struct Chunk {
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
}

impl Chunk {
    pub(crate) fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let chunk = Chunk{ chunk_type: chunk_type, chunk_data: data };
        return chunk;
    }

    fn length(&self) -> u32 {
        self.chunk_data.len().try_into().expect("Length is too large to fit in a u32")
    }

    pub(crate) fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    
    fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    fn crc(&self) -> u32 {
        const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let type_bytes = self.chunk_type().bytes();
        let all_bytes = [type_bytes.as_slice(), self.data()].concat();
        CRC32.checksum(all_bytes.as_slice())
    }

    pub(crate) fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data().to_vec())
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
         let mut bytes_vec = self.length().to_be_bytes().to_vec();
         bytes_vec.extend_from_slice(self.chunk_type().bytes().as_slice());
         bytes_vec.extend_from_slice(self.data());
         bytes_vec.extend_from_slice(self.crc().to_be_bytes().as_slice());

         return bytes_vec;
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        let data_len: u32 = u32::from_be_bytes(value[..4].try_into().expect("Chunk length slice should be of length 4"));

        let chunk_type_bytes: [u8; 4] = value[4..8].try_into().expect("Chunk type slice should be of length 4");

        let chunk_type = match ChunkType::try_from(chunk_type_bytes) {
            Ok(chunk_type) => chunk_type,
            Err(error) => panic!("{error:?}")
        };

        let end_of_data_index:usize = usize::try_from(8+data_len).unwrap();

        let value_vec = value[8..end_of_data_index].to_vec();

        let new_chunk = Chunk{ chunk_type: chunk_type, chunk_data: value_vec };

        let crc = u32::from_be_bytes(value[end_of_data_index..].try_into().expect("Chunk crc slice should be of length 4"));

        if new_chunk.crc() != crc {
            return Err("Crc doesn't match".to_string());
        }

        return Ok(new_chunk);
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.data_as_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
