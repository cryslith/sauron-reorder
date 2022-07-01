use sauron::{html::attributes::style, prelude::*};

pub enum Msg {
  RedClicked,
  BlueClicked,
  Swap,
}

pub struct App {
  swap: bool,
}

impl App {
  pub fn new() -> Self {
    Self { swap: false }
  }
}

impl Application<Msg> for App {
  fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
    match msg {
      Msg::Swap => {
        log!("swap");
        self.swap = !self.swap;
      }
      Msg::RedClicked => {
        log!("red");
      }
      Msg::BlueClicked => {
        log!("blue");
      }
    }
    Cmd::none()
  }

  fn view(&self) -> Node<Msg> {
    let mut my_buttons: Vec<Node<Msg>> = vec![
      button(
        [
          attr("data-disambiguate", "red"), // suggested fix
          on_click(|_| Msg::RedClicked),
        ],
        [text!("Red")],
      ),
      button(
        [
          attr("data-disambiguate", "blue"), // suggested fix
          on_click(|_| Msg::BlueClicked),
        ],
        [text!("Blue")],
      ),
    ];
    if self.swap {
      my_buttons.reverse();
    }
    article(
      [],
      [div(
        [
          style("display", "flex"),
          style("align-items", "center"),
          style("flex-direction", "column"),
        ],
        [
          div(
            [
              style("display", "flex"),
              style("align-items", "center"),
              style("flex-direction", "row"),
            ],
            my_buttons,
          ),
          button([on_click(|_| Msg::Swap)], [text!("Swap")]),
        ],
      )],
    )
  }
}
