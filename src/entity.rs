use crate::action;

pub trait Entity
{
    fn play_turn(&mut self) -> action::Action;
    fn take_dmg(&mut self, val : i32) -> bool;
}
