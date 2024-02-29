trait Attacker {
    fn choose_style(&self) -> String;
}
#[allow(dead_code, unused_variables)]
#[derive(Debug)]
enum Character {
    Worrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Worrior => "Wing Chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "thai chi".to_string(),
        }
    }
}
#[allow(dead_code, unused_variables)]
#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_traits() {
        let my_character = Character::Archer;
        let chosen_fighting_Style = my_character.choose_style();
        dbg!(chosen_fighting_Style);
    }
}
