mod style;

use std::path::PathBuf;

use iced::image::Handle as ImageHandle;
use iced::{button, scrollable, slider, text_input};
use iced::{
    Align, Button, Checkbox, Color, Column, Container, Element, HorizontalAlignment, Image, Length,
    Radio, Row, Sandbox, Scrollable, Settings, Slider, Space, Text, TextInput,
};
use subprocess::Exec;

struct Easel {
    theme: style::Theme,
    scroll: scrollable::State,
    src_btn: button::State,
    src_img: Option<PathBuf>,
    img_handle: ImageHandle,
}

#[derive(Debug, Clone)]
enum Event {
    SourcePressed,
    ThemeChanged(style::Theme),
}

impl Sandbox for Easel {
    type Message = Event;

    fn new() -> Self {
        Self {
            theme: style::Theme::Dark,
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
            Event::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
    }

    fn view(&mut self) -> Element<Event> {
        let choose_img = Button::new(
            &mut self.src_btn,
            Text::new("Choose image").horizontal_alignment(HorizontalAlignment::Center),
        )
        .padding(14)
        .on_press(Event::SourcePressed)
        .style(self.theme);

        let choose_theme = Row::new()
            .max_width(200)
            .push(
                Radio::new(
                    style::Theme::Dark,
                    &format!("{:?}", style::Theme::Dark),
                    Some(self.theme),
                    Event::ThemeChanged,
                )
                .style(self.theme),
            )
            .push(
                Radio::new(
                    style::Theme::Light,
                    &format!("{:?}", style::Theme::Light),
                    Some(self.theme),
                    Event::ThemeChanged,
                )
                .style(self.theme),
            );

        let header = Row::new()
            .padding(14)
            .push(choose_img)
            .push(Space::with_width(Length::Fill))
            .push(choose_theme);

        let image = Container::new(Image::new(self.img_handle.clone())).padding(14);

        let content = Column::new()
            .spacing(10)
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(header)
            .push(image);

        let scrollable = Scrollable::new(&mut self.scroll).push(
            Container::new(content)
                .width(Length::Fill)
                .center_x()
                .style(self.theme),
        );

        Container::new(scrollable)
            .height(Length::Fill)
            .style(self.theme)
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
