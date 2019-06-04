pub enum UiComponent {
  FillBox { fill : char }
}

pub fn render(ui : UiComponent, width : u32, height : u32) {
  use UiComponent::*;
  match ui {
    FillBox { fill } => for i in 0..height {
      for j in 0..width {
        print!("{}", fill);
      }
      print!("\n");
    }
  }
}
