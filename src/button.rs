use macroquad::audio::load_sound;
use macroquad::{
    audio::Sound,
    prelude::*,
    ui::{root_ui, Skin, Style},
};

#[allow(dead_code)]
pub struct MenuButtons {
    // window_background: Image,
    // button_background:,
    // button_clicked_background:,
    // font: Vec<u8>,
    pub window_style: Style,
    pub button_style: Style,
    pub label_style: Style,
    // ui_skin: Skin,
    pub sound: Sound,
}

#[allow(dead_code)]
impl MenuButtons {
    pub fn new(ws: Style, bs: Style, ls: Style, sound: Sound) -> Self {
        MenuButtons {
            // window_background: wb,
            // font: f,
            window_style: ws,
            button_style: bs,
            label_style: ls,
            // ui_skin: skin,
            sound,
        }
    }

    // pub fn get_sound(self) -> Sound {
    //     self.sound
    // }
}

pub async fn load_window_background(path: &str) -> Image {
    // load_image("assets/Solid_black.png").await.unwrap();
    load_image(path).await.unwrap()
}

// pub async fn button_background() -> Image {
//     load_image("assets/green_button.png").await.unwrap();
// }

// pub async fn button_clicked_background() -> Image {
//     load_image("assets/pressed_button.png").await.unwrap();
// }

pub async fn load_font(font_path: &str) -> Vec<u8> {
    load_file(font_path).await.unwrap()
}

pub async fn load_window_style(window_background: Image) -> Style {
    root_ui()
        .style_builder()
        .background(window_background)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build()
}

pub async fn load_button_style(font: Vec<u8>) -> Style {
    root_ui()
        .style_builder()
        // .background(button_background)
        // .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .font(&font)
        .unwrap()
        .text_color(BLACK)
        .font_size(64)
        .build()
}

pub async fn load_label_style(font: Vec<u8>) -> Style {
    root_ui()
        .style_builder()
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(28)
        .build()
}

#[allow(dead_code)]
pub async fn load_ui_skin(window_style: Style, button_style: Style, label_style: Style) -> Skin {
    Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    }
}

pub async fn loading_sound(sound_path: &str) -> Sound {
    load_sound(sound_path).await.unwrap()
}

// pub async fn click_sound() {
//     let click = load_sound("assets/computer-mouse-click-352734.wav").await.unwrap();
//     play_sound(&click, PlaySoundParams { looped: false, volume: 0.1 });
// }
