mod Entity
{
    pub trait Entity
    {
        fn play_turn(mut &self);
        fn take_dmg(mut &self, val : i32) -> bool;
    }
}