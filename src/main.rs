mod action;
mod entity;
mod ui;

fn main() {
    println!("Hello, world!");

    let act : action::Action_type;

    ui::render(ui::UiComponent::FillBox{ fill : 'X' }, 10, 5);
}
