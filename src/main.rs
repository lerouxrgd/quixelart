use std::path::PathBuf;

use iced::image::Handle as ImageHandle;
use iced::{button, scrollable, slider, text_input};
use iced::{
    Button, Checkbox, Color, Column, Container, Element, HorizontalAlignment, Image, Length, Radio,
    Row, Sandbox, Scrollable, Settings, Slider, Space, Text, TextInput,
};
use subprocess::Exec;

struct Easel {
    scroll: scrollable::State,
    src_btn: button::State,
    src_img: Option<PathBuf>,
    img_handle: ImageHandle,
}

#[derive(Debug, Clone)]
enum Event {
    SourcePressed,
}

impl Sandbox for Easel {
    type Message = Event;

    fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            src_btn: button::State::new(),
            src_img: None,
            img_handle: ImageHandle::from_memory(vec![]),
        }
    }

    fn title(&self) -> String {
        "QuixelArt".into()
    }

    fn update(&mut self, evt: Event) {
        match evt {
            Event::SourcePressed => {
                self.src_img = match nfd2::open_file_dialog(None, None).unwrap() {
                    nfd2::Response::Okay(file_path) => Some(file_path),
                    nfd2::Response::OkayMultiple(_) | nfd2::Response::Cancel => None,
                };

                if let Some(path_buf) = &self.src_img {
                    let img_bytes = make_img(&path_buf.as_path().to_string_lossy());
                    self.img_handle = ImageHandle::from_memory(img_bytes);
                }
            }
        }
    }

    fn view(&mut self) -> Element<Event> {
        // let mut content = Row::new();
        let mut content = Column::new();

        let a = Column::new()
            .spacing(20)
            .width(Length::Fill)
            .push(Text::new("Image").size(34))
            .push(Container::new(Image::new(self.img_handle.clone())))
            .push(
                Button::new(
                    &mut self.src_btn,
                    Text::new("Choose image").horizontal_alignment(HorizontalAlignment::Center),
                )
                .padding(12)
                .min_width(100)
                .on_press(Event::SourcePressed),
            );

        let b = Column::new()
            .spacing(20)
            .width(Length::Fill)
            .push(Text::new("Placeholder").size(18));

        content = content.push(a).push(b);

        let scrollable = Scrollable::new(&mut self.scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

fn make_img(src: &str) -> Vec<u8> {
    (Exec::cmd("magick")
        .arg("convert")
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
        | Exec::cmd("magick")
            .arg("convert")
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
