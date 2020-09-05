mod style;

use std::path::PathBuf;

use iced::image::Handle as ImageHandle;
use iced::{button, scrollable, slider};
use iced::{
    Align, Button, Checkbox, Color, Column, Container, Element, Font, HorizontalAlignment, Image,
    Length, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space, Text, VerticalAlignment,
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
    pixelize_slider: slider::State,
    pixelize: u8,
    kcolors_slider: slider::State,
    kcolors: u8,
    level_black_slider: slider::State,
    level_black: u8,
    level_white_slider: slider::State,
    level_white: u8,
}

#[derive(Debug, Clone)]
enum Event {
    SourcePressed,
    ThemeChanged(style::Theme),
    SliderPixelizeChanged(u8),
    SliderPixelizeReleased,
    SliderKcolorsChanged(u8),
    SliderKcolorsReleased,
    SliderLevelBlackChanged(u8),
    SliderLevelBlackReleased,
    SliderLevelWhiteChanged(u8),
    SliderLevelWhiteReleased,
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
            pixelize_slider: slider::State::new(),
            pixelize: 80,
            kcolors_slider: slider::State::new(),
            kcolors: 32,
            level_black_slider: slider::State::new(),
            level_black: 10,
            level_white_slider: slider::State::new(),
            level_white: 80,
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
            Event::SliderPixelizeChanged(pixelize) => {
                self.pixelize = pixelize;
            }
            Event::SliderPixelizeReleased => {
                self.make_img();
            }
            Event::SliderKcolorsChanged(kcolors) => {
                self.kcolors = kcolors;
            }
            Event::SliderKcolorsReleased => {
                self.make_img();
            }
            Event::SliderLevelBlackChanged(level_black) => {
                self.level_black = level_black;
            }
            Event::SliderLevelBlackReleased => {
                self.make_img();
            }
            Event::SliderLevelWhiteChanged(level_white) => {
                self.level_white = level_white;
            }
            Event::SliderLevelWhiteReleased => {
                self.make_img();
            }
        }
    }

    fn view(&mut self) -> Element<Event> {
        const PADDING: u16 = 14;

        let choose_img = Button::new(&mut self.src_button, Text::new("Choose image"))
            .padding(PADDING)
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
            .padding(PADDING)
            .push(choose_img)
            .push(Space::with_width(Length::Fill))
            .push(choose_theme);

        let image = Container::new(Image::new(self.img_handle.clone())).padding(PADDING);

        let name_width = 120;
        let val_width = 50;

        let pixelize = Row::new()
            .padding(PADDING)
            .spacing(10)
            .push(Text::new("Pixelize").width(Length::Units(name_width)))
            .push(
                Slider::new(
                    &mut self.pixelize_slider,
                    0..=99,
                    self.pixelize,
                    Event::SliderPixelizeChanged,
                )
                .on_release(Event::SliderPixelizeReleased)
                .width(Length::Fill)
                .style(self.theme),
            )
            .push(
                Text::new(&format!("{} %", self.pixelize))
                    .width(Length::Units(val_width))
                    .font(FONT_PIX_L),
            );

        let kcolors = Row::new()
            .padding(PADDING)
            .spacing(10)
            .push(Text::new("Colors").width(Length::Units(name_width)))
            .push(
                Slider::new(
                    &mut self.kcolors_slider,
                    1..=64,
                    self.kcolors,
                    Event::SliderKcolorsChanged,
                )
                .on_release(Event::SliderKcolorsReleased)
                .width(Length::Fill)
                .style(self.theme),
            )
            .push(
                Text::new(self.kcolors.to_string())
                    .width(Length::Units(val_width))
                    .font(FONT_PIX_L),
            );

        let level_black = Row::new()
            .spacing(10)
            .push(Text::new("black").width(Length::Units(name_width)))
            .push(
                Slider::new(
                    &mut self.level_black_slider,
                    0..=100,
                    self.level_black,
                    Event::SliderLevelBlackChanged,
                )
                .on_release(Event::SliderLevelBlackReleased)
                .width(Length::Fill)
                .style(self.theme),
            )
            .push(
                Text::new(format!("{} %", self.level_black))
                    .width(Length::Units(val_width))
                    .font(FONT_PIX_L),
            );

        let level_white = Row::new()
            .spacing(10)
            .push(Text::new("white").width(Length::Units(name_width)))
            .push(
                Slider::new(
                    &mut self.level_white_slider,
                    0..=100,
                    self.level_white,
                    Event::SliderLevelWhiteChanged,
                )
                .on_release(Event::SliderLevelWhiteReleased)
                .width(Length::Fill)
                .style(self.theme),
            )
            .push(
                Text::new(format!("{} %", self.level_white))
                    .width(Length::Units(val_width))
                    .font(FONT_PIX_L),
            );

        let levels = Row::new()
            .padding(PADDING)
            .spacing(10)
            .push(Text::new("Levels").width(Length::Units(name_width)))
            .push(Column::new().push(level_black).push(level_white));

        let content = Column::new()
            .spacing(5)
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(header)
            .push(image)
            .push(pixelize)
            .push(kcolors)
            .push(levels);

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
            pixelize,
            kcolors,
            level_black,
            level_white,
            ..
        } = self;

        if let Some(img_path) = img_path.as_ref().map(PathBuf::as_path) {
            let img_bytes = (Exec::cmd("magick")
                .arg("convert")
                .arg(img_path.to_string_lossy().as_ref())
                .arg("-resize")
                .arg(format!("{}%", 100 - *pixelize))
                .arg("-level")
                .arg(format!("{}%,{}%", level_black, level_white))
                .arg("-")
                | Exec::cmd("magick")
                    .arg("-")
                    .arg("-kmeans")
                    .arg(kcolors.to_string())
                    .arg("-")
                | Exec::cmd("magick")
                    .arg("convert")
                    .arg("-")
                    .arg("-filter")
                    .arg("point")
                    .arg("-resize")
                    .arg(format!("{}%", 1.0 / (100 - *pixelize) as f32 * 10_000.0))
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
