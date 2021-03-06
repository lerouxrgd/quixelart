mod style;

use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::Once;

use iced::image::Handle as ImageHandle;
use iced::{button, scrollable, slider};
use iced::{
    Align, Button, Checkbox, Column, Container, Element, Font, HorizontalAlignment, Image, Length,
    Row, Sandbox, Scrollable, Settings, Slider, Space, Text, VerticalAlignment,
};
use iced_native::widget::image::Data as ImageData;
use magick_rust::{bindings as magick, magick_wand_genesis, MagickWand};

static INIT_IMAGE_MAGICK: Once = Once::new();

const FONT_PIXEL: Font = Font::External {
    name: "Pixel",
    bytes: include_bytes!("../fonts/Pixel.ttf"),
};

const FONT_PIX_L: Font = Font::External {
    name: "PIX-L",
    bytes: include_bytes!("../fonts/PIX-L.ttf"),
};

const FONT_ICONS: Font = Font::External {
    name: "Pixel",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

struct Easel {
    theme: style::Theme,
    layout: Layout,
    src_button: button::State,
    src_path: Option<PathBuf>,
    layout_button: button::State,
    theme_button: button::State,
    img_handle: ImageHandle,
    scroll: scrollable::State,
    pixelize_slider: slider::State,
    pixelize: u8,
    kcolors_slider: slider::State,
    kcolors: u8,
    level_toggle: bool,
    level_black_slider: slider::State,
    level_black: u8,
    level_white_slider: slider::State,
    level_white: u8,
    modulate_toggle: bool,
    modulate_brightness_slider: slider::State,
    modulate_brightness: u8,
    modulate_saturation_slider: slider::State,
    modulate_saturation: u8,
    modulate_hue_slider: slider::State,
    modulate_hue: u8,
    save_button: button::State,
    save_as_button: button::State,
    save_path: Option<PathBuf>,
    save_file: Option<PathBuf>,
    saved: bool,
}

#[derive(Debug, Clone)]
enum Event {
    SourcePressed,
    LayoutPressed,
    ThemePressed,
    SliderPixelizeChanged(u8),
    SliderPixelizeReleased,
    SliderKcolorsChanged(u8),
    SliderKcolorsReleased,
    LevelToggled(bool),
    SliderLevelBlackChanged(u8),
    SliderLevelBlackReleased,
    SliderLevelWhiteChanged(u8),
    SliderLevelWhiteReleased,
    ModulateToggled(bool),
    SliderModulateBrightnessChanged(u8),
    SliderModulateBrightnessReleased,
    SliderModulateSaturationChanged(u8),
    SliderModulateSaturationReleased,
    SliderModulateHueChanged(u8),
    SliderModulateHueReleased,
    SavePressed,
    SaveAsPressed,
}

#[derive(Debug, Clone)]
enum Layout {
    Columns,
    Rows,
}

impl Layout {
    fn swap(&mut self) {
        match self {
            Self::Columns => *self = Self::Rows,
            Self::Rows => *self = Self::Columns,
        }
    }
}

impl Sandbox for Easel {
    type Message = Event;

    fn new() -> Self {
        Self {
            theme: style::Theme::Dark,
            layout: Layout::Columns,
            src_button: button::State::new(),
            src_path: None,
            layout_button: button::State::new(),
            theme_button: button::State::new(),
            img_handle: ImageHandle::from_memory(vec![]),
            scroll: scrollable::State::new(),
            pixelize_slider: slider::State::new(),
            pixelize: 80,
            kcolors_slider: slider::State::new(),
            kcolors: 32,
            level_toggle: true,
            level_black_slider: slider::State::new(),
            level_black: 10,
            level_white_slider: slider::State::new(),
            level_white: 80,
            modulate_toggle: false,
            modulate_brightness_slider: slider::State::new(),
            modulate_brightness: 100,
            modulate_saturation_slider: slider::State::new(),
            modulate_saturation: 100,
            modulate_hue_slider: slider::State::new(),
            modulate_hue: 100,
            save_button: button::State::new(),
            save_as_button: button::State::new(),
            save_path: None,
            save_file: None,
            saved: false,
        }
    }

    fn title(&self) -> String {
        match self.save_file.as_ref().map(|f| f.file_name()) {
            Some(Some(file_name)) => {
                let saved = if self.saved { " " } else { "*" };
                format!("QuixelArt - {} {}", file_name.to_string_lossy(), saved)
            }
            _ => "QuixelArt".into(),
        }
    }

    fn update(&mut self, evt: Event) {
        match evt {
            Event::LayoutPressed => {
                self.layout.swap();
            }
            Event::ThemePressed => {
                self.theme.swap();
            }
            Event::SourcePressed => {
                let file_path = rfd::FileDialog::new()
                    // .add_filter("rust", &["rs", "toml"])
                    // .set_directory(&path)
                    .pick_file();
                self.src_path = file_path.clone();

                if let Some(mut file_path) = file_path {
                    file_path.pop();
                    self.save_path = Some(file_path);
                    self.save_file = None;
                }

                self.make_img();
            }
            Event::SliderPixelizeChanged(pixelize) => {
                self.pixelize = pixelize;
            }
            Event::SliderKcolorsChanged(kcolors) => {
                self.kcolors = kcolors;
            }
            Event::SliderPixelizeReleased | Event::SliderKcolorsReleased => {
                self.make_img();
            }
            Event::LevelToggled(level_toggle) => {
                self.level_toggle = level_toggle;
                self.make_img();
            }
            Event::SliderLevelBlackChanged(level_black) => {
                self.level_black = level_black;
            }
            Event::SliderLevelWhiteChanged(level_white) => {
                self.level_white = level_white;
            }
            Event::SliderLevelBlackReleased | Event::SliderLevelWhiteReleased => {
                if self.level_toggle {
                    self.make_img();
                }
            }
            Event::ModulateToggled(modulate_toggle) => {
                self.modulate_toggle = modulate_toggle;
                self.make_img();
            }
            Event::SliderModulateBrightnessChanged(modulate_brightness) => {
                self.modulate_brightness = modulate_brightness;
            }
            Event::SliderModulateSaturationChanged(modulate_saturation) => {
                self.modulate_saturation = modulate_saturation;
            }
            Event::SliderModulateHueChanged(modulate_hue) => {
                self.modulate_hue = modulate_hue;
            }
            Event::SliderModulateBrightnessReleased
            | Event::SliderModulateSaturationReleased
            | Event::SliderModulateHueReleased => {
                if self.modulate_toggle {
                    self.make_img();
                }
            }
            Event::SavePressed | Event::SaveAsPressed => {
                let select_file = (matches!(evt, Event::SavePressed) && self.save_file.is_none())
                    || matches!(evt, Event::SaveAsPressed);

                if select_file {
                    let default_path = self.save_path.as_ref().map(PathBuf::as_path);

                    let mut save_file = rfd::FileDialog::new();
                    if let Some(default_path) = default_path {
                        save_file = save_file.set_directory(default_path);
                    }
                    let save_file = save_file.save_file();

                    save_file.as_ref().map(|f| {
                        let mut save_path = f.clone();
                        save_path.pop();
                        self.save_path = Some(save_path);
                    });

                    self.save_file = save_file;
                }

                let save_wtr = self.save_file.as_ref().map(|f| {
                    fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .create(true)
                        .open(f)
                        .ok()
                });

                match (save_wtr, &self.img_handle.data()) {
                    (Some(Some(ref mut save_wtr)), ImageData::Bytes(bytes)) => {
                        if save_wtr.write_all(bytes).is_ok() {
                            self.saved = true;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn view(&mut self) -> Element<Event> {
        const PADDING: u16 = 5;

        let choose_img = Button::new(&mut self.src_button, choose_img_icon())
            .on_press(Event::SourcePressed)
            .style(self.theme);

        let mut save_img = Button::new(&mut self.save_button, save_img_icon()).style(self.theme);
        let mut save_img_as =
            Button::new(&mut self.save_as_button, save_img_as_icon()).style(self.theme);

        if self.save_path.is_some() {
            save_img = save_img.on_press(Event::SavePressed);
            save_img_as = save_img_as.on_press(Event::SaveAsPressed);
        }

        let change_theme = Button::new(&mut self.theme_button, theme_icon(&self.theme))
            .on_press(Event::ThemePressed)
            .style(self.theme);

        let change_layout = Button::new(&mut self.layout_button, layout_icon(&self.layout))
            .on_press(Event::LayoutPressed)
            .style(self.theme);

        let header = Row::new()
            .padding(PADDING)
            .spacing(5)
            .align_items(Align::Center)
            .push(choose_img)
            .push(save_img_as)
            .push(save_img)
            .push(Space::with_width(Length::Fill))
            .push(change_layout)
            .push(change_theme)
            .push(Space::with_width(Length::Units(5)));

        let main_name_width = 115;
        let sub_name_width = 105;
        let val_width = 50;

        let pixelize = Row::new()
            .padding(PADDING)
            .spacing(10)
            .push(Text::new("Pixelize").width(Length::Units(main_name_width)))
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
            .push(Text::new("Colors").width(Length::Units(main_name_width)))
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
            .push(Text::new("black").width(Length::Units(sub_name_width)))
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
            .push(Text::new("white").width(Length::Units(sub_name_width)))
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

        let mut levels = Row::new().padding(PADDING).spacing(10).push(
            Checkbox::new(self.level_toggle, "Levels", Event::LevelToggled)
                .width(Length::Units(main_name_width))
                .spacing(10)
                .style(self.theme),
        );

        if self.level_toggle {
            levels = levels.push(Column::new().push(level_black).push(level_white));
        } else {
            levels = levels.push(Space::with_width(Length::Fill));
        }

        let modulate_brightness = Row::new()
            .spacing(10)
            .push(Text::new("brightness").width(Length::Units(sub_name_width)))
            .push(
                Slider::new(
                    &mut self.modulate_brightness_slider,
                    0..=200,
                    self.modulate_brightness,
                    Event::SliderModulateBrightnessChanged,
                )
                .on_release(Event::SliderModulateBrightnessReleased)
                .width(Length::Fill)
                .style(self.theme),
            )
            .push(
                Text::new(self.modulate_brightness.to_string())
                    .width(Length::Units(val_width))
                    .font(FONT_PIX_L),
            );

        let modulate_saturation = Row::new()
            .spacing(10)
            .push(Text::new("saturation").width(Length::Units(sub_name_width)))
            .push(
                Slider::new(
                    &mut self.modulate_saturation_slider,
                    0..=200,
                    self.modulate_saturation,
                    Event::SliderModulateSaturationChanged,
                )
                .on_release(Event::SliderModulateSaturationReleased)
                .width(Length::Fill)
                .style(self.theme),
            )
            .push(
                Text::new(self.modulate_saturation.to_string())
                    .width(Length::Units(val_width))
                    .font(FONT_PIX_L),
            );

        let modulate_hue = Row::new()
            .spacing(10)
            .push(Text::new("hue").width(Length::Units(sub_name_width)))
            .push(
                Slider::new(
                    &mut self.modulate_hue_slider,
                    0..=200,
                    self.modulate_hue,
                    Event::SliderModulateHueChanged,
                )
                .on_release(Event::SliderModulateHueReleased)
                .width(Length::Fill)
                .style(self.theme),
            )
            .push(
                Text::new(self.modulate_hue.to_string())
                    .width(Length::Units(val_width))
                    .font(FONT_PIX_L),
            );

        let mut modulate = Row::new().padding(PADDING).spacing(10).push(
            Checkbox::new(self.modulate_toggle, "Modulate", Event::ModulateToggled)
                .width(Length::Units(main_name_width))
                .spacing(10)
                .style(self.theme),
        );

        if self.modulate_toggle {
            modulate = modulate.push(
                Column::new()
                    .push(modulate_brightness)
                    .push(modulate_saturation)
                    .push(modulate_hue),
            );
        } else {
            modulate = modulate.push(Space::with_width(Length::Fill))
        }

        let controls_length = match self.layout {
            Layout::Columns => Length::Units(420),
            Layout::Rows => Length::Fill,
        };

        let controls = Column::new()
            .spacing(5)
            .align_items(Align::Center)
            .width(controls_length)
            .push(header)
            .push(pixelize)
            .push(kcolors)
            .push(levels)
            .push(modulate);

        let image = Container::new(Image::new(self.img_handle.clone()))
            .padding(PADDING)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .width(Length::Fill);

        let content = match self.layout {
            Layout::Columns => {
                let content = Row::new().padding(PADDING).push(controls).push(image);
                Container::new(content)
            }
            Layout::Rows => {
                let content = Column::new().padding(PADDING).push(image).push(controls);
                Container::new(content)
            }
        };
        let content = Scrollable::new(&mut self.scroll)
            .scroller_width(5)
            .push(content);

        Container::new(
            Column::new()
                .push(content)
                .push(Space::new(Length::Fill, Length::Fill)),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .style(self.theme)
        .into()
    }
}

impl Easel {
    fn make_img(&mut self) {
        let Easel {
            src_path,
            img_handle,
            pixelize,
            kcolors,
            level_toggle,
            level_black,
            level_white,
            modulate_toggle,
            modulate_brightness,
            modulate_saturation,
            modulate_hue,
            saved,
            ..
        } = self;

        let src_path = match src_path.as_ref().map(PathBuf::as_path) {
            Some(src_path) => src_path,
            None => return,
        };

        let wand = MagickWand::new();

        wand.read_image(src_path.to_string_lossy().as_ref()).ok();
        let width = wand.get_image_width();
        let height = wand.get_image_height();

        let downsize = (100.0 - *pixelize as f64) / 100.0;
        let width_ds = ((width as f64) * downsize).round() as usize;
        let height_ds = ((height as f64) * downsize).round() as usize;
        wand.resize_image(width_ds, height_ds, magick::FilterType_UndefinedFilter);

        if *level_toggle {
            wand.level_image(
                *level_black as f64 / 100.0,
                1.0,
                *level_white as f64 / 100.0,
            )
            .ok();
        }

        if *modulate_toggle {
            wand.modulate_image(
                *modulate_brightness as f64,
                *modulate_saturation as f64,
                *modulate_hue as f64,
            )
            .ok();
        }

        wand.kmeans(*kcolors as usize, 100, 0.01).ok();

        wand.resize_image(width, height, magick::FilterType_PointFilter);

        if let Ok(img_bytes) = wand.write_image_blob("png") {
            *img_handle = ImageHandle::from_memory(img_bytes);
            *saved = false;
        }
    }
}

fn icon(unicode: char, size: u16) -> Text {
    Text::new(&unicode.to_string())
        .font(FONT_ICONS)
        .height(Length::Units(20))
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .size(size)
}

fn choose_img_icon() -> Text {
    icon('\u{e802}', 20)
}

fn save_img_icon() -> Text {
    icon('\u{e803}', 20)
}

fn save_img_as_icon() -> Text {
    icon('\u{f1c5}', 20)
}

fn layout_icon(layout: &Layout) -> Text {
    let code = match layout {
        Layout::Columns => '\u{f152}',
        Layout::Rows => '\u{f151}',
    };

    icon(code, 20)
}

fn theme_icon(theme: &style::Theme) -> Text {
    let (code, size) = match theme {
        style::Theme::Dark => ('\u{e800}', 30),
        style::Theme::Light => ('\u{e801}', 20),
    };

    icon(code, size)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() >= 2 && (args[1] == "-V" || args[1] == "--version") {
        let version = option_env!("CARGO_PKG_VERSION").unwrap_or_else(|| ".");
        println!("v{}", version);
        return Ok(());
    }

    INIT_IMAGE_MAGICK.call_once(|| {
        magick_wand_genesis();
    });

    let mut settings = Settings::default();
    settings.default_text_size = 18;
    if let Font::External { bytes, .. } = FONT_PIXEL {
        settings.default_font = Some(bytes);
    }

    Easel::run(settings)?;

    Ok(())
}
