use core::array::TryFromSliceError;
use pkhex_rs_macros::{data_get_set_proc, byte_parser_proc};

use crate::utils::SliceUtils;

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
        section_info: Some(SectionData::new(section_bytes)),
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

macro_rules! byte_parser {
    ($($field_name:ident : $type:ty => $offset:expr ; $endianess:ident),+) => {
        $(
            paste::paste! {
                pub fn [<get_ $field_name>](data: &[u8]) -> $type {
                    SliceUtils::[<get_ $type _ $endianess _offset>](data, $offset).unwrap()
                }

                pub fn [<set_ $field_name>](data: &mut [u8], value: $type) {
                    let value_bytes = $type::[<to_ $endianess _bytes>](value);
                    data.write_into(&value_bytes, $offset);
                }
            }
        )+
    };
}


#[macro_export]
macro_rules! byte_struct_test {
    ($name:ident => $($field_name:ident: $type:tt $_:tt $offset:tt | $endianess:ident),+) => {
        use $crate::utils::SliceUtils;

        #[derive(Debug)]
        pub struct $name {
            $(
                pub $field_name: $type,
            )+
        }

        paste::paste! {
            impl $name {
                pub fn new(data: &[u8]) -> Self {
                    $name {
                        $(
                            $field_name: $name::[<get_ $field_name>](&data),
                        )+
                    }
                }

                $(
                    pub fn [<get_ $field_name>](data: &[u8]) -> $type {
                        SliceUtils::[<get_ $type _ $endianess _offset>](data, $offset).unwrap()
                    }
                )+
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! bytes_get_set {
    ($var_name:ident : $type:ty; $data:ident $(;get => $getter:block)? $(;set, $value:ident => $setter:block)?) => {
        paste::paste! {
            $(
                pub fn [<get_ $var_name _from_bytes>]($data: &[u8]) -> $type $getter
            )?

            $(
                pub fn [<set_ $var_name> _to_bytes]($data: &mut [u8], $value: $type) $setter
            )?
        }
    }
}

macro_rules! self_get_set {
    ($var_name:ident : $type:ty; $self:ident $(;set, $value:ident => $setter:expr)?) => {
        paste::paste! {
            pub fn [<get_ $var_name>](&$self) -> &$type {
                &$self.$var_name
            }

            $(
                pub fn [<set_ $var_name>](&mut $self,  $value: $type) {
                    $self.$var_name = $value
                }
            )?
        }
    }
}

impl TrainerId {
    byte_parser_proc! { trainer_id : u32@0x000A }
    byte_parser_proc! { secret_id: u16@0x000A#le, public_id: u16@0x000A+2 }
}

impl PlayedTime {
    byte_parser! { 
        hours: u32 => 0x000E;le,
        minutes: u32 => 0x000E+2;le,
        seconds: u32 => 0x000E+3;le,
        frames: u32 => 0x000E+4;le
    }

    // Example without proc-macros
    // bytes_get_set! { time: [u8; 5]; data;
    //     get => { <[u8; 5]>::try_from(data.get_offset(0x000E, 5)).unwrap() }
    // }

    data_get_set_proc! { time: [u8; 5];
        get => { <[u8; 5]>::try_from(data.get_offset(0x000E, 5)).unwrap() }
    }

    self_get_set! { time: [u8; 5]; self;
        set, value => value
    }
}

#[cfg(test)]
mod tests {
    use crate::save::gen3::gen3_save::{TrainerId, PlayedTime};

    #[test]
    fn exploration() {
        let file_bytes = match std::fs::read("assets/test.sav") {
                Ok(bytes) => bytes,
                Err(e) => panic!("{}", e)
            };
        
        // let save_a_bytes = file_bytes.get_offset(0x0, 57344);
        let save_b_bytes = file_bytes.get_offset(0x00E000, 57344);
    
        // Real world scenario where we can't guarantee that a save file will be passed
        // let mut data: Option<&[u8]> = None;
        // for i in 0..14 {
        //     let offset_for_section_to_be = 4096*i;
        //     let current_section = save_b_bytes.get_offset(offset_for_section_to_be, 4096);
        //     let section_id = if let Ok(id) = current_section.get_u16_le_offset(0x0FF4) {
        //         id
        //     } else {
        //         panic!("Could not found section ID at offset: {}", offset_for_section_to_be);
        //     };
            
    
        //     if section_id == 0 {
        //         data = Some(current_section);
        //         break;
        //     }
        // }

        // let data = if let Some(inner_data) = data {
        //     inner_data
        // } else {
        //     panic!("The section 0 was not found");
        // };

        // Because we are in a test, we can guarantee that this save will always have the section 0
        let mut counter = 0;
        let data = loop {
            let current_section = save_b_bytes.get_offset(4096*counter, 4096);
            let section_id = current_section.get_u16_le_offset(0x0FF4).unwrap();
    
            if section_id == 0 {
                break current_section;
            }

            counter += 1;
            if counter > 13 {
                panic!("We looped through all the sections and the ID 0 was not found");
            }
        };

        const A: usize = 0x0FF6;
        byte_struct_test! {
            SectionData => section_id: u16@0x0FF4|le, checksum: u16@A|le, signature: u32@0x0FF8|le, save_index: u32@0x0FFC|le
        }

        let fn_test = SectionData {
            section_id: data.get_u16_le_offset(0x0FF4).unwrap(),
            checksum: data.get_u16_le_offset(0x0FF6).unwrap(),
            signature: data.get_u32_le_offset(0x0FF8).unwrap(),
            save_index: data.get_u32_le_offset(0x0FFC).unwrap(),
        };
        
        let macro_test = SectionData::new(data);

        let proc = TrainerId::get_trainer_id_from_bytes(data);
        let direct = data.get_u32_le_offset(0x000A).unwrap();
        assert_eq!(proc, direct, "Proc: {}, Direct: {}", proc, direct);

        PlayedTime::get_time_from_bytes(data);

        assert_eq!(macro_test.section_id, fn_test.section_id);
        assert_eq!(macro_test.checksum, fn_test.checksum);
        assert_eq!(macro_test.signature, fn_test.signature);
        assert_eq!(macro_test.save_index, fn_test.save_index);

        assert_eq!(SectionData::get_section_id(data), fn_test.section_id);
        assert_eq!(SectionData::get_checksum(data), fn_test.checksum);
        assert_eq!(SectionData::get_signature(data), fn_test.signature);
        assert_eq!(SectionData::get_save_index(data), fn_test.save_index);

    }
}