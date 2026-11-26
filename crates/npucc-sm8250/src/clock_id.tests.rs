use super::*;
use crate::{ClockBranch, ClockSource, LucidPll, PostDivider};

#[test]
fn each_device_tree_clock_has_exactly_one_provider_kind() {
    for (index, id) in ClockId::ALL.into_iter().enumerate() {
        assert_eq!(id as usize, index);
        assert_eq!(ClockId::from_index(index), Some(id));

        let kinds = [
            ClockBranch::for_id(id).is_some(),
            ClockSource::for_id(id).is_some(),
            LucidPll::for_id(id).is_some(),
            PostDivider::for_id(id).is_some(),
        ];
        assert_eq!(kinds.into_iter().filter(|matches| *matches).count(), 1);
    }

    assert_eq!(ClockId::from_index(ClockId::ALL.len()), None);
}
