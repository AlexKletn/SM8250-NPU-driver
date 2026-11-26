use super::GraphCommand;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphReply {
    pub firmware_status: u32,
    pub handle: u32,
}

impl GraphReply {
    pub const HEADER_BYTES: usize = 24;

    pub fn parse(packet: &[u8], command: GraphCommand, transaction: u32) -> Option<Self> {
        if packet.len() < Self::HEADER_BYTES
            || !packet.len().is_multiple_of(4)
            || (command == GraphCommand::Load && packet.len() != Self::HEADER_BYTES)
        {
            return None;
        }
        let mut words = packet
            .chunks_exact(4)
            .map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
        let [size, opcode, firmware_status, reply_transaction, _, handle] =
            core::array::from_fn(|_| words.next().unwrap_or_default());
        if size as usize != packet.len()
            || opcode != command.reply_opcode()
            || reply_transaction != transaction
        {
            return None;
        }
        Some(Self {
            firmware_status,
            handle,
        })
    }
}

#[cfg(test)]
#[path = "graph_reply.tests.rs"]
mod tests;
