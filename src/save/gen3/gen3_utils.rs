use std::array::TryFromSliceError;

use crate::utils::{little_endian_u8_to_u32, little_endian_u8_to_u16, SliceUtils};
use super::gen3_save::{Gen3Game, KeyCode, SaveGen3, SectionData, TrainerData, TrainerId, PlayedTime};

pub fn get_security_key_or_game_code(bytes_a: &[u8], bytes_b: &[u8], save: &mut SaveGen3) -> Result<KeyCode, TryFromSliceError> {
    let block_a = little_endian_u8_to_u32(bytes_a)?;
    let block_b = little_endian_u8_to_u32(bytes_b)?;

    if block_a == 0x00000000 {
        save.game_ver = Gen3Game::RubySapphire;
        Ok(KeyCode {
            game_code: Some(0x00000000),
            security_key: None
        })
    } else if block_a == 0x00000001 {
        save.game_ver = Gen3Game::FireRedLeafGreen;
        Ok(KeyCode {
            game_code: Some(0x00000001),
            security_key: Some(block_b)
        })
    } else {
        save.game_ver = Gen3Game::Emerald;
        Ok(KeyCode {
            game_code: None,
            security_key: Some(block_a)
        })
    }
}

pub fn parse_trainer_data_from_byte_array(section_bytes: &[u8], save: &mut SaveGen3) -> Result<TrainerData, TryFromSliceError> {
    Ok(TrainerData {
        section_info: Some(SectionData {
            section_id: little_endian_u8_to_u16(section_bytes.get_offset(0x0FF4, 2))?,
            checksum: little_endian_u8_to_u16(section_bytes.get_offset(0x0FF6, 2))?,
            signature: little_endian_u8_to_u32(section_bytes.get_offset(0x0FF8, 4))?,
            save_index: little_endian_u8_to_u32(section_bytes.get_offset(0x0FFC, 4))?,
        }),
        name: <[u8; 7]>::try_from(section_bytes.get_offset(0x0, 7))?,
        gender: section_bytes[8],
        id: TrainerId {
            trainer_id: little_endian_u8_to_u32(section_bytes.get_offset(0x000A, 4))?,
            secret_id: little_endian_u8_to_u16(section_bytes.get_offset(0x000A, 2))?,
            public_id: little_endian_u8_to_u16(section_bytes.get_offset(0x000A+2, 2))?,
        },
        time: PlayedTime {
            time: <[u8; 5]>::try_from(section_bytes.get_offset(0x000E, 5))?,
            hours: little_endian_u8_to_u16(section_bytes.get_offset(0x000E, 2))?,
            minutes: section_bytes[0x000E+2],
            seconds: section_bytes[0x000E+3],
            frames: section_bytes[0x000E+4],
        },
        security: get_security_key_or_game_code(
            section_bytes.get_offset(0x00AC, 4),
            section_bytes.get_offset(0x0AF8, 4), save)?,
    })
}
