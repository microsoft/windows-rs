#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CharacterGrouping();
    fn CharacterGroupings();
    fn ICharacterGrouping();
    fn ICharacterGroupings();
    fn ICharacterGroupingsFactory();
}
