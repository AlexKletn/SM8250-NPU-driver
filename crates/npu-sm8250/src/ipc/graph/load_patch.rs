#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadPatch {
    words: [u32; 6],
}

impl LoadPatch {
    pub const fn from_firmware_words(words: [u32; 6]) -> Self {
        Self { words }
    }

    pub(super) const fn words(self) -> [u32; 6] {
        self.words
    }
}
