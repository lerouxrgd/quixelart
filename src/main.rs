mod style;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use iced::image::Handle as ImageHandle;
use iced::{button, pane_grid, scrollable, slider};
use iced::{
    Align, Button, Checkbox, Color, Column, Container, Element, Font, HorizontalAlignment, Image,
    Length, PaneGrid, Radio, Row, Sandbox, Scrollable, Settings, Slider, Space, Text,
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

struct QuixelArt {
    panes: pane_grid::State<Content>,
    theme: Rc<RefCell<style::Theme>>,
    src_button: button::State,
    easel: Rc<RefCell<Easel>>,
    palette: Rc<RefCell<Palette>>,
}

enum Content {
    Easel {
        easel: Rc<RefCell<Easel>>,
        state: EaselState,
    },
    Palette {
        palette: Rc<RefCell<Palette>>,
        state: PaletteState,
    },
}

struct Easel {
    img_path: Option<PathBuf>,
    img_handle: ImageHandle,
}

struct EaselState {
    scroll: scrollable::State,
}

struct Palette {
    theme: Rc<RefCell<style::Theme>>,
    pixelize: u8,
    kcolors: u8,
    level_toggle: bool,
    level_black: u8,
    level_white: u8,
    modulate_toggle: bool,
    modulate_brightness: u8,
    modulate_saturation: u8,
    modulate_hue: u8,
}

struct PaletteState {
    scroll: scrollable::State,
    pixelize_slider: slider::State,
    kcolors_slider: slider::State,
    level_black_slider: slider::State,
    level_white_slider: slider::State,
    modulate_brightness_slider: slider::State,
    modulate_saturation_slider: slider::State,
    modulate_hue_slider: slider::State,
}

#[derive(Debug, Clone)]
enum Event {
    SourcePressed,
    ThemeChanged(style::Theme),
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
}

impl Sandbox for QuixelArt {
    type Message = Event;

    fn new() -> Self {
        let theme = Rc::new(RefCell::new(style::Theme::Dark));

        let easel = Rc::new(RefCell::new(Easel {
            img_path: None,
            img_handle: ImageHandle::from_memory(vec![]),
        }));

        let easel_state = EaselState {
            scroll: scrollable::State::new(),
        };

        let easel_content = Content::Easel {
            easel: easel.clone(),
            state: easel_state,
        };

        let palette = Rc::new(RefCell::new(Palette {
            theme: theme.clone(),
            pixelize: 80,
            kcolors: 32,
            level_toggle: true,
            level_black: 10,
            level_white: 80,
            modulate_toggle: false,
            modulate_brightness: 100,
            modulate_saturation: 100,
            modulate_hue: 100,
        }));

        let palette_state = PaletteState {
            scroll: scrollable::State::new(),
            pixelize_slider: slider::State::new(),
            kcolors_slider: slider::State::new(),
            level_black_slider: slider::State::new(),
            level_white_slider: slider::State::new(),
            modulate_brightness_slider: slider::State::new(),
            modulate_saturation_slider: slider::State::new(),
            modulate_hue_slider: slider::State::new(),
        };

        let palette_content = Content::Palette {
            palette: palette.clone(),
            state: palette_state,
        };

        let (mut panes, pane) = pane_grid::State::new(easel_content);
        panes.split(pane_grid::Axis::Vertical, &pane, palette_content);

        Self {
            panes,
            theme,
            src_button: button::State::new(),
            easel,
            palette,
        }
    }

    fn title(&self) -> String {
        "QuixelArt".into()
    }

    fn update(&mut self, evt: Event) {
        match evt {
            Event::ThemeChanged(theme) => {
                *self.theme.borrow_mut() = theme;
            }
            Event::SourcePressed => {
                self.easel.borrow_mut().img_path = match nfd2::open_file_dialog(None, None).unwrap()
                {
                    nfd2::Response::Okay(file_path) => Some(file_path),
                    nfd2::Response::OkayMultiple(_) | nfd2::Response::Cancel => None,
                };
                self.make_img();
            }
            Event::SliderPixelizeChanged(pixelize) => {
                self.palette.borrow_mut().pixelize = pixelize;
            }
            Event::SliderKcolorsChanged(kcolors) => {
                self.palette.borrow_mut().kcolors = kcolors;
            }
            Event::SliderPixelizeReleased | Event::SliderKcolorsReleased => {
                self.make_img();
            }
            Event::LevelToggled(level_toggle) => {
                self.palette.borrow_mut().level_toggle = level_toggle;
                self.make_img();
            }
            Event::SliderLevelBlackChanged(level_black) => {
                self.palette.borrow_mut().level_black = level_black;
            }
            Event::SliderLevelWhiteChanged(level_white) => {
                self.palette.borrow_mut().level_white = level_white;
            }
            Event::SliderLevelBlackReleased | Event::SliderLevelWhiteReleased => {
                if self.palette.borrow().level_toggle {
                    self.make_img();
                }
            }
            Event::ModulateToggled(modulate_toggle) => {
                self.palette.borrow_mut().modulate_toggle = modulate_toggle;
                self.make_img();
            }
            Event::SliderModulateBrightnessChanged(modulate_brightness) => {
                self.palette.borrow_mut().modulate_brightness = modulate_brightness;
            }
            Event::SliderModulateSaturationChanged(modulate_saturation) => {
                self.palette.borrow_mut().modulate_saturation = modulate_saturation;
            }
            Event::SliderModulateHueChanged(modulate_hue) => {
                self.palette.borrow_mut().modulate_hue = modulate_hue;
            }
            Event::SliderModulateBrightnessReleased
            | Event::SliderModulateSaturationReleased
            | Event::SliderModulateHueReleased => {
                if self.palette.borrow().modulate_toggle {
                    self.make_img();
                }
            }
        }
    }

    fn view(&mut self) -> Element<Event> {
        const PADDING: u16 = 14;
        let theme = self.theme.borrow().clone();

        let choose_img = Button::new(&mut self.src_button, Text::new("Choose image"))
            .padding(PADDING)
            .on_press(Event::SourcePressed)
            .style(theme);

        let choose_theme = Row::new()
            .max_width(200)
            .push(
                Radio::new(
                    style::Theme::Dark,
                    &format!("{:?}", style::Theme::Dark),
                    Some(theme),
                    Event::ThemeChanged,
                )
                .size(20)
                .spacing(5)
                .style(theme),
            )
            .push(Space::with_width(Length::Units(8)))
            .push(
                Radio::new(
                    style::Theme::Light,
                    &format!("{:?}", style::Theme::Light),
                    Some(theme),
                    Event::ThemeChanged,
                )
                .size(20)
                .spacing(5)
                .style(theme),
            );

        let header = Row::new()
            .padding(PADDING)
            .push(choose_img)
            .push(Space::with_width(Length::Fill))
            .push(choose_theme);

        let name_width = 120;
        let val_width = 50;

        let pane_grid = PaneGrid::new(&mut self.panes, |_pane, content| match content {
            Content::Easel { easel, state } => {
                let image =
                    Container::new(Image::new(easel.borrow().img_handle.clone())).padding(PADDING);

                let content = Column::new()
                    .align_items(Align::Center)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .push(image);

                // let scrollable = Scrollable::new(&mut state.scroll).push(
                //     content
                //     // Container::new(content)
                //     //     .width(Length::Fill)
                //     //     .height(Length::Fill)
                //     //     .center_x()
                //     //     .center_y()
                //     //     .style(theme),
                // );

                let title_bar = pane_grid::TitleBar::new("Easel").padding(10);

                pane_grid::Content::new(content).title_bar(title_bar)
            }
            Content::Palette { palette, state } => {
                let theme = palette.borrow().theme.borrow().clone();

                let pixelize = Row::new()
                    .padding(PADDING)
                    .spacing(10)
                    .push(Text::new("Pixelize").width(Length::Units(name_width)))
                    .push(
                        Slider::new(
                            &mut state.pixelize_slider,
                            0..=99,
                            palette.borrow().pixelize,
                            Event::SliderPixelizeChanged,
                        )
                        .on_release(Event::SliderPixelizeReleased)
                        .width(Length::Fill)
                        .style(theme),
                    )
                    .push(
                        Text::new(&format!("{} %", palette.borrow().pixelize))
                            .width(Length::Units(val_width))
                            .font(FONT_PIX_L),
                    );

                let kcolors = Row::new()
                    .padding(PADDING)
                    .spacing(10)
                    .push(Text::new("Colors").width(Length::Units(name_width)))
                    .push(
                        Slider::new(
                            &mut state.kcolors_slider,
                            1..=64,
                            palette.borrow().kcolors,
                            Event::SliderKcolorsChanged,
                        )
                        .on_release(Event::SliderKcolorsReleased)
                        .width(Length::Fill)
                        .style(theme),
                    )
                    .push(
                        Text::new(palette.borrow().kcolors.to_string())
                            .width(Length::Units(val_width))
                            .font(FONT_PIX_L),
                    );

                let level_black = Row::new()
                    .spacing(10)
                    .push(Text::new("black").width(Length::Units(name_width)))
                    .push(
                        Slider::new(
                            &mut state.level_black_slider,
                            0..=100,
                            palette.borrow().level_black,
                            Event::SliderLevelBlackChanged,
                        )
                        .on_release(Event::SliderLevelBlackReleased)
                        .width(Length::Fill)
                        .style(theme),
                    )
                    .push(
                        Text::new(format!("{} %", palette.borrow().level_black))
                            .width(Length::Units(val_width))
                            .font(FONT_PIX_L),
                    );

                let level_white = Row::new()
                    .spacing(10)
                    .push(Text::new("white").width(Length::Units(name_width)))
                    .push(
                        Slider::new(
                            &mut state.level_white_slider,
                            0..=100,
                            palette.borrow().level_white,
                            Event::SliderLevelWhiteChanged,
                        )
                        .on_release(Event::SliderLevelWhiteReleased)
                        .width(Length::Fill)
                        .style(theme),
                    )
                    .push(
                        Text::new(format!("{} %", palette.borrow().level_white))
                            .width(Length::Units(val_width))
                            .font(FONT_PIX_L),
                    );

                let mut levels = Row::new().padding(PADDING).spacing(10).push(
                    Checkbox::new(palette.borrow().level_toggle, "Levels", Event::LevelToggled)
                        .width(Length::Units(name_width))
                        .style(theme),
                );

                if palette.borrow().level_toggle {
                    levels = levels.push(Column::new().push(level_black).push(level_white));
                } else {
                    levels = levels.push(Space::with_width(Length::Fill));
                }

                let modulate_brightness = Row::new()
                    .spacing(10)
                    .push(Text::new("brightness").width(Length::Units(name_width)))
                    .push(
                        Slider::new(
                            &mut state.modulate_brightness_slider,
                            0..=200,
                            palette.borrow().modulate_brightness,
                            Event::SliderModulateBrightnessChanged,
                        )
                        .on_release(Event::SliderModulateBrightnessReleased)
                        .width(Length::Fill)
                        .style(theme),
                    )
                    .push(
                        Text::new(palette.borrow().modulate_brightness.to_string())
                            .width(Length::Units(val_width))
                            .font(FONT_PIX_L),
                    );

                let modulate_saturation = Row::new()
                    .spacing(10)
                    .push(Text::new("saturation").width(Length::Units(name_width)))
                    .push(
                        Slider::new(
                            &mut state.modulate_saturation_slider,
                            0..=200,
                            palette.borrow().modulate_saturation,
                            Event::SliderModulateSaturationChanged,
                        )
                        .on_release(Event::SliderModulateSaturationReleased)
                        .width(Length::Fill)
                        .style(theme),
                    )
                    .push(
                        Text::new(palette.borrow().modulate_saturation.to_string())
                            .width(Length::Units(val_width))
                            .font(FONT_PIX_L),
                    );

                let modulate_hue = Row::new()
                    .spacing(10)
                    .push(Text::new("hue").width(Length::Units(name_width)))
                    .push(
                        Slider::new(
                            &mut state.modulate_hue_slider,
                            0..=200,
                            palette.borrow().modulate_hue,
                            Event::SliderModulateHueChanged,
                        )
                        .on_release(Event::SliderModulateHueReleased)
                        .width(Length::Fill)
                        .style(theme),
                    )
                    .push(
                        Text::new(palette.borrow().modulate_hue.to_string())
                            .width(Length::Units(val_width))
                            .font(FONT_PIX_L),
                    );

                let mut modulate = Row::new().padding(PADDING).spacing(10).push(
                    Checkbox::new(
                        palette.borrow().modulate_toggle,
                        "Modulate",
                        Event::ModulateToggled,
                    )
                    .width(Length::Units(name_width))
                    .style(theme),
                );

                if palette.borrow().modulate_toggle {
                    modulate = modulate.push(
                        Column::new()
                            .push(modulate_brightness)
                            .push(modulate_saturation)
                            .push(modulate_hue),
                    );
                } else {
                    modulate = modulate.push(Space::with_width(Length::Fill))
                }

                let title_bar = pane_grid::TitleBar::new("Palette").padding(10);

                let content = Column::new()
                    .spacing(5)
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .push(pixelize)
                    .push(kcolors)
                    .push(levels)
                    .push(modulate);

                let scrollable = Scrollable::new(&mut state.scroll).push(
                    content
                    // Container::new(content)
                    //     .width(Length::Fill)
                    //     .height(Length::Fill)
                    //     .center_x()
                    //     .style(theme),
                );

                pane_grid::Content::new(scrollable).title_bar(title_bar)
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);

        let content = Column::new()
            .spacing(5)
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(header)
            .push(pane_grid);

        Container::new(content)
            .height(Length::Fill)
            .style(theme)
            .into()
    }
}

impl QuixelArt {
    fn make_img(&self) {
        let Easel {
            img_path,
            img_handle,
        } = &mut *self.easel.borrow_mut();

        let Palette {
            pixelize,
            kcolors,
            level_toggle,
            level_black,
            level_white,
            modulate_toggle,
            modulate_brightness,
            modulate_saturation,
            modulate_hue,
            ..
        } = &*self.palette.borrow();

        if let Some(img_path) = img_path.as_ref().map(PathBuf::as_path) {
            let mut downsize = Exec::cmd("magick")
                .arg("convert")
                .arg(img_path.to_string_lossy().as_ref())
                .arg("-resize")
                .arg(format!("{}%", 100 - pixelize));

            if *level_toggle {
                downsize = downsize
                    .arg("-level")
                    .arg(format!("{}%,{}%", level_black, level_white));
            }

            if *modulate_toggle {
                downsize = downsize.arg("-modulate").arg(format!(
                    "{},{},{}",
                    modulate_brightness, modulate_saturation, modulate_hue
                ))
            }

            downsize = downsize.arg("-");

            let kmeans = Exec::cmd("magick")
                .arg("-")
                .arg("-kmeans")
                .arg(kcolors.to_string())
                .arg("-");

            let upsize = Exec::cmd("magick")
                .arg("convert")
                .arg("-")
                .arg("-filter")
                .arg("point")
                .arg("-resize")
                .arg(format!("{}%", 1.0 / (*pixelize) as f32 * 10_000.0))
                .arg("-");

            let img_bytes = (downsize | kmeans | upsize).capture().unwrap().stdout;

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

    QuixelArt::run(settings).unwrap();
}
