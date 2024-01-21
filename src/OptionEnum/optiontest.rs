pub fn option_test() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);
    return opt1;
}

pub fn option_test_string() -> Option<String> {
    let mut opt2: Option<String> = None;
    opt2 = Some("some string name".to_string());
    return opt2;
}

pub enum CharacterType {
    Archer,
    Warrior,
    Mage
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer",
            CharacterType::Mage => "Mage",
            _ => "Warrior"
        }.to_string()
    }
}

pub fn option_test_char() -> Option<CharacterType> {
    let mut char_type: Option<CharacterType> = None;
    char_type = Some(CharacterType::Mage);
    return char_type;
}
