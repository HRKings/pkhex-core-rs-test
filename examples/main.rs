use pkhex_rs::{utils::{SliceUtils}, save::gen3::{gen3_save::{SaveGen3, Gen3Game}, gen3_utils::parse_trainer_data_from_byte_array}};

fn main() {

  let file_bytes = match std::fs::read("assets/test.sav") {
    Ok(bytes) => bytes,
    Err(e) => panic!("{}", e)
  };

  // let save_a_bytes = file_bytes.get_offset(0x0, 57344);
  let save_b_bytes = file_bytes.get_offset(0x00E000, 57344);

  let mut save = SaveGen3 {
      trainer_section: None,
      game_ver: Gen3Game::RubySapphire,
  };

  for i in 0..14 {
      let current_section = save_b_bytes.get_offset(4096*i, 4096);
      let section_id = current_section.get_u16_le_offset(0x0FF4).unwrap();

      match section_id {
          0 => save.trainer_section = Some(parse_trainer_data_from_byte_array(current_section, &mut save).unwrap()),
          _ => println!("The section ID is {}", section_id)
      };
  }

  println!("{:#?}", save);
}
