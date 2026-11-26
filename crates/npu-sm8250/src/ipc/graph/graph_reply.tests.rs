use super::*;
use crate::ipc::graph::{Execute, ExecutionBuffer, Unload};

#[test]
fn execute_packet_contains_buffer_addresses_and_patch_ids() {
    let command = Execute {
        handle: 42,
        buffers: &[
            ExecutionBuffer {
                iova: 0x1000,
                patch_id: 0,
            },
            ExecutionBuffer {
                iova: 0x2000,
                patch_id: 0xd71000,
            },
        ],
    };
    let mut bytes = [0; 40];
    assert_eq!(command.encode(3, &mut bytes).unwrap().len(), 40);
    let words = bytes
        .chunks_exact(4)
        .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()));
    assert!(words.eq([40, 0x0a, 3, 0, 42, 2, 0x1000, 0, 0x2000, 0xd71000]));
    assert_eq!(Unload::encode(4, 42)[16..20], 42_u32.to_le_bytes());
}

#[test]
fn reply_validates_command_and_transaction_and_preserves_status() {
    let mut bytes = [0; 24];
    for (word, dest) in [24_u32, 0x10001, 7, 3, 0, 42]
        .into_iter()
        .zip(bytes.chunks_exact_mut(4))
    {
        dest.copy_from_slice(&word.to_le_bytes());
    }
    assert_eq!(
        GraphReply::parse(&bytes, GraphCommand::Load, 3),
        Some(GraphReply {
            firmware_status: 7,
            handle: 42
        })
    );
    assert_eq!(GraphReply::parse(&bytes, GraphCommand::Unload, 3), None);
    assert_eq!(GraphReply::parse(&bytes, GraphCommand::Load, 4), None);
}
