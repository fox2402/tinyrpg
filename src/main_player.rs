use crate::action;

struct Main_player
{
    max_health : i32,
    mut current_health : i32,

    base_dmg : i32,
}

impl entity::Entity for Main_player
{
    fn play_turn(&self) -> action::Action
    {
        action::Action;
    }

    fn take_dmg(&self, val : i32) -> bool
    {
        self.current_health = self.current_health - val;
        self.current_health < 0;
    }
}
