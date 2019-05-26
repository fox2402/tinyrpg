use crate::entity;

pub enum Action_type
{
    Attack,
    Heal,
    Defend,
}

pub struct Action
{
    pub act_type : Action_type,
    pub val : i32,
    pub target : entity::Entity,
}
