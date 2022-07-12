use core::array::TryFromSliceError;
use crate::utils::{SliceUtils};
use super::gen3_save::{Gen3Game, KeyCode, SaveGen3, SectionData, TrainerData, TrainerId, PlayedTime};

pub fn get_security_key_or_game_code(block_a: u32, block_b: u32, save: &mut SaveGen3) -> KeyCode {
    if block_a == 0x00000000 {
        save.game_ver = Gen3Game::RubySapphire;
        KeyCode {
            game_code: Some(0x00000000),
            security_key: None
        }
    } else if block_a == 0x00000001 {
        save.game_ver = Gen3Game::FireRedLeafGreen;
        KeyCode {
            game_code: Some(0x00000001),
            security_key: Some(block_b)
        }
    } else {
        save.game_ver = Gen3Game::Emerald;
        KeyCode {
            game_code: None,
            security_key: Some(block_a)
        }
    }
}

pub fn parse_trainer_data_from_byte_array(section_bytes: &[u8], save: &mut SaveGen3) -> Result<TrainerData, TryFromSliceError> {
    Ok(TrainerData {
        section_info: Some(SectionData {
            section_id: section_bytes.get_u16_le_offset(0x0FF4)?,
            checksum: section_bytes.get_u16_le_offset(0x0FF6)?,
            signature: section_bytes.get_u32_le_offset(0x0FF8)?,
            save_index: section_bytes.get_u32_le_offset(0x0FFC)?,
        }),
        name: <[u8; 7]>::try_from(section_bytes.get_offset(0x0, 7))?,
        gender: section_bytes[8],
        id: TrainerId {
            trainer_id: section_bytes.get_u32_le_offset(0x000A)?,
            secret_id: section_bytes.get_u16_le_offset(0x000A)?,
            public_id: section_bytes.get_u16_le_offset(0x000A+2)?,
        },
        time: PlayedTime {
            time: <[u8; 5]>::try_from(section_bytes.get_offset(0x000E, 5))?,
            hours: section_bytes.get_u16_le_offset(0x000E)?,
            minutes: section_bytes[0x000E+2],
            seconds: section_bytes[0x000E+3],
            frames: section_bytes[0x000E+4],
        },
        security: get_security_key_or_game_code(
            section_bytes.get_u32_le_offset(0x00AC)?,
            section_bytes.get_u32_le_offset(0x0AF8)?, save),
    })
}
