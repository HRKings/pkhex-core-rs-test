use std::fmt::{Display, Formatter, Result};

use crate::byte_struct_test;

#[derive(Debug)]
pub enum Gen3Game {
    RubySapphire,
    FireRedLeafGreen,
    Emerald,
}

impl Display for Gen3Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Gen3Game::RubySapphire => write!(f, "Ruby/Sapphire"),
            Gen3Game::FireRedLeafGreen => write!(f, "FireRed/LeafGreen"),
            Gen3Game::Emerald => write!(f, "Emerald"),
        }
    }
}

#[derive(Debug)]
pub struct KeyCode {
    pub game_code: Option<u32>,
    pub security_key: Option<u32>,
}

#[derive(Debug)]
pub struct PlayedTime {
    pub time: [u8; 5],
    pub hours: u16,
    pub minutes: u8,
    pub seconds: u8,
    pub frames: u8,
}

#[derive(Debug)]
pub struct TrainerId {
    pub trainer_id: u32,
    pub secret_id: u16,
    pub public_id: u16,
}

#[derive(Debug)]
pub struct TrainerData {
    pub section_info: Option<SectionData>,
    pub name: [u8; 7],
    pub gender: u8,
    pub id: TrainerId,
    pub time: PlayedTime,
    pub security: KeyCode,
}

byte_struct_test! {
    SectionData => section_id: u16@0x0FF4|le, checksum: u16@0x0FF6|le, signature: u32@0x0FF8|le, save_index: u32@0x0FFC|le
}

impl Display for SectionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "ID: {}, Checksum: {}, Signature: {}, Save Index: {}",
            self.section_id, self.checksum, self.signature, self.save_index
        )
    }
}

#[derive(Debug)]
pub struct SaveGen3 {
    pub game_ver: Gen3Game,
    pub trainer_section: Option<TrainerData>,
}
