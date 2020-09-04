mod style;

use std::path::PathBuf;

use iced::image::Handle as ImageHandle;
use iced::{button, scrollable, slider, text_input};
use iced::{
    Align, Button, Checkbox, Color, Column, Container, Element, Font, HorizontalAlignment, Image,
    Length, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space, Text, TextInput,
    VerticalAlignment,
};
use subprocess::Exec;

const FONT_PIXEL: Font = Font::External {
    name: "Pixel",
    bytes: include_bytes!("../fonts/Pixel.ttf"),
};

const FONT_PIX_L: Font = Font::External {
    name: "PIX-L",
    bytes: include_bytes!("../fonts/PIX-L.ttf"),
};

struct Easel {
    theme: style::Theme,
    scroll: scrollable::State,
    src_button: button::State,
    img_path: Option<PathBuf>,
    img_handle: ImageHandle,
    resize_slider: slider::State,
    resize_val: u8,
}

#[derive(Debug, Clone)]
enum Event {
    SourcePressed,
    ThemeChanged(style::Theme),
    SliderResizeChanged(u8),
    SliderResizeReleased,
}

impl Sandbox for Easel {
    type Message = Event;

    fn new() -> Self {
        Self {
            theme: style::Theme::Dark,
            scroll: scrollable::State::new(),
            src_button: button::State::new(),
            img_path: None,
            img_handle: ImageHandle::from_memory(vec![]),
            resize_slider: slider::State::new(),
            resize_val: 80,
        }
    }

    fn title(&self) -> String {
        "QuixelArt".into()
    }

    fn update(&mut self, evt: Event) {
        match evt {
            Event::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Event::SourcePressed => {
                self.img_path = match nfd2::open_file_dialog(None, None).unwrap() {
                    nfd2::Response::Okay(file_path) => Some(file_path),
                    nfd2::Response::OkayMultiple(_) | nfd2::Response::Cancel => None,
                };
                self.make_img();
            }
            Event::SliderResizeChanged(resize_val) => {
                self.resize_val = resize_val;
            }
            Event::SliderResizeReleased => {
                self.make_img();
            }
        }
    }

    fn view(&mut self) -> Element<Event> {
        let choose_img = Button::new(&mut self.src_button, Text::new("Choose image"))
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
                .spacing(5)
                .style(self.theme),
            )
            .push(Space::with_width(Length::Units(8)))
            .push(
                Radio::new(
                    style::Theme::Light,
                    &format!("{:?}", style::Theme::Light),
                    Some(self.theme),
                    Event::ThemeChanged,
                )
                .spacing(5)
                .style(self.theme),
            );

        let header = Row::new()
            .padding(14)
            .push(choose_img)
            .push(Space::with_width(Length::Fill))
            .push(choose_theme);

        let image = Container::new(Image::new(self.img_handle.clone())).padding(14);

        let resize = Row::new()
            .padding(14)
            .spacing(10)
            .push(Text::new("Pixelization").width(Length::Units(120)))
            .push(
                Slider::new(
                    &mut self.resize_slider,
                    0..=99,
                    self.resize_val,
                    Event::SliderResizeChanged,
                )
                .on_release(Event::SliderResizeReleased)
                .width(Length::Fill)
                .style(self.theme),
            )
            .push(
                Text::new(&format!("{} %", self.resize_val))
                    .width(Length::Units(50))
                    .font(FONT_PIX_L),
            );

        let content = Column::new()
            .spacing(10)
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(header)
            .push(image)
            .push(resize);

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

impl Easel {
    fn make_img(&mut self) {
        let Easel {
            img_path,
            img_handle,
            resize_val,
            ..
        } = self;

        if let Some(img_path) = img_path.as_ref().map(PathBuf::as_path) {
            let img_bytes = (Exec::cmd("magick")
                .arg("convert")
                .arg(img_path.to_string_lossy().as_ref())
                .arg("-resize")
                .arg(format!("{}%", 100 - *resize_val))
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
                    .arg(format!("{}%", 1.0 / (100 - *resize_val) as f32 * 10_000.0))
                    .arg("-"))
            .capture()
            .unwrap()
            .stdout;

            *img_handle = ImageHandle::from_memory(img_bytes);
        }
    }
}

fn main() {
    let mut settings = Settings::default();

    settings.default_text_size = 18;

    if let Font::External { bytes, .. } = FONT_PIXEL {
        settings.default_font = Some(bytes);
    }

    Easel::run(settings)
}
