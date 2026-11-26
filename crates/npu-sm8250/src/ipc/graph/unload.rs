use super::GraphCommand;

pub struct Unload;

impl Unload {
    pub fn encode(transaction: u32, handle: u32) -> [u8; 20] {
        let mut packet = [0; 20];
        let _ = GraphCommand::Unload.encode(transaction, 1, [handle].into_iter(), &mut packet);
        packet
    }
}
