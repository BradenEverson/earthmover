//! Implementation of communication protocol on the board itself with the Mega peripheral reader

/// A communication packet for Board to Board peripheral communication
#[derive(Clone, Copy, Debug)]
pub struct MoverPacket {
    pub version: u8,
    pub msg_type: MessageType,
    pub len: u16,
}

impl MoverPacket {
    pub fn new(version: u8, msg_type: MessageType, len: u16) -> Self {
        Self {
            version,
            msg_type,
            len,
        }
    }

    pub fn v1_from_data(msg_type: MessageType, data: &[u8]) -> Self {
        Self {
            version: 1,
            msg_type,
            len: data.len() as u16,
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let len_split = self.len.to_be_bytes();
        [
            self.version,
            self.msg_type as u8,
            len_split[0],
            len_split[1],
        ]
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 4 {
            None
        } else {
            let version = bytes[0];
            let msg_type_byte = bytes[1];
            let msg_type = MessageType::from_byte(msg_type_byte)?;
            let split = [bytes[2], bytes[3]];
            let len = u16::from_be_bytes(split);

            Some(Self::new(version, msg_type, len))
        }
    }

    pub fn serialize_completely(&self, data: &[u8]) -> Vec<u8> {
        let mut res = vec![];
        let bytes = self.to_bytes();
        res.extend(bytes.iter());
        res.extend(data.iter());

        res
    }
}

/// Message header type, essentially what type of data to expect from the packet's payload
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum MessageType {
    Lidar,
    Accelerometer,
    //... implement more as needed
}

impl MessageType {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Lidar),
            1 => Some(Self::Accelerometer),
            _ => None,
        }
    }
}

/// Reads an entire stream of bytes as tuples between MoverPackets and their payloads
pub fn data_stream_to_packet_pairs(input: &[u8]) -> Option<Vec<(MoverPacket, &[u8])>> {
    let mut res = vec![];
    let mut reader = input;

    while reader.len() >= 4 {
        let packet = MoverPacket::from_bytes(&reader[0..4])?;
        let len = packet.len;
        let data = &reader[0..(4 + len as usize)];
        res.push((packet, data));
        reader = &reader[(4 + len as usize)..];
    }

    Some(res)
}

/// Blocks and reads an entire message packet and data pair from an external source
pub fn read_packet() -> (MoverPacket, Vec<u8>) {
    todo!("Chrissy and I need to work on this part together, this is where we'll interface with the Mega");
}
