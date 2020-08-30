use std::path::PathBuf;

use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Color, Column, Container, Element,
    HorizontalAlignment, Image, Length, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space,
    Text, TextInput,
};
use subprocess::Exec;

struct Easel {
    scroll: scrollable::State,
    src_btn: button::State,
    src_img: Option<PathBuf>,
}

#[derive(Debug, Clone)]
enum Msg {
    SourcePressed,
}

impl Sandbox for Easel {
    type Message = Msg;

    fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            src_btn: button::State::new(),
            src_img: None,
        }
    }

    fn title(&self) -> String {
        "QuixelArt".into()
    }

    fn update(&mut self, evt: Msg) {
        match evt {
            Msg::SourcePressed => {
                self.src_img = match nfd2::open_file_dialog(None, None).unwrap() {
                    nfd2::Response::Okay(file_path) => Some(file_path),
                    nfd2::Response::OkayMultiple(_) | nfd2::Response::Cancel => None,
                };
            }
        }
    }

    fn view(&mut self) -> Element<Msg> {
        let img_bytes = match &self.src_img {
            Some(path_buf) => make_img(&path_buf.as_path().to_string_lossy()),
            None => vec![],
        };
        let img = iced::image::Handle::from_memory(img_bytes);

        let content = Column::new()
            .spacing(20)
            .push(Text::new("Image").size(50))
            .push(Container::new(Image::new(img)))
            .push(
                Button::new(
                    &mut self.src_btn,
                    Text::new("Select image").horizontal_alignment(HorizontalAlignment::Center),
                )
                .padding(12)
                .min_width(100)
                .on_press(Msg::SourcePressed),
            );

        let scrollable = Scrollable::new(&mut self.scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

fn make_img(src: &str) -> Vec<u8> {
    (Exec::cmd("convert")
        .arg(src)
        .arg("-resize")
        .arg("20%")
        .arg("-level")
        .arg("10%,80%")
        .arg("-")
        | Exec::cmd("magick")
            .arg("-")
            .arg("-kmeans")
            .arg("32")
            .arg("-")
        | Exec::cmd("convert")
            .arg("-")
            .arg("-filter")
            .arg("point")
            .arg("-resize")
            .arg("500%")
            .arg("-"))
    .capture()
    .unwrap()
    .stdout
}

fn main() {
    Easel::run(Settings::default())
}
