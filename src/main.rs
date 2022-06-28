use crate::utils::{little_endian_u8_to_u16, SliceUtils};
use crate::save::gen3::gen3_save::{Gen3Game, SaveGen3};
use crate::save::gen3::gen3_utils::parse_trainer_data_from_byte_array;

mod utils;
mod save;

fn main() {
    let file_bytes = match std::fs::read("assets/test.sav") {
      Ok(bytes) => bytes,
      Err(e) => panic!("{}", e)
  };

    // let save_a_bytes = file_bytes.get_offset(0, 57344);
    let save_b_bytes = file_bytes.get_offset(0x00E000, 57344);

    let mut save = SaveGen3 {
        trainer_section: None,
        game_ver: Gen3Game::RubySapphire,
    };

    for i in 0..14 {
        let current_section = save_b_bytes.get_offset(4096*i, 4096);
        let section_id = little_endian_u8_to_u16(current_section.get_offset(0x0FF4, 2)).unwrap();

        match section_id {
            0 => save.trainer_section = Some(parse_trainer_data_from_byte_array(current_section, &mut save).unwrap()),
            _ => println!("The section ID is {}", section_id)
        };
    }

    let trainer_section = save.trainer_section.unwrap();
    println!("The save is from: {}", save.game_ver);
    println!("The trainer section info is: ({}), the trainer name is: {:X?} and their gender is: {}", trainer_section.section_info.unwrap(), trainer_section.name, trainer_section.gender);
    println!("The time played is {} hours, {} minutes, {} seconds and {} frames", trainer_section.time.hours, trainer_section.time.minutes, trainer_section.time.seconds, trainer_section.time.frames);
}
