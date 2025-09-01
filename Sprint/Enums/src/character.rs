pub fn test_option_string ()-> Option<CharacterType>{
  let mut chartype = None;
  chartype = Some(CharacterType::Archer);
  return chartype;
}

pub enum CharacterType{
  Archer,
  Warrior,
  Mage,
}