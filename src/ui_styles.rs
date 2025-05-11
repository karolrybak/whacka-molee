// karolrybak/whacka-molee/whacka-molee-351ea95a23ebe18bf0ffbcd3437412c5de79bebd/src/ui_styles.rs
// version:0.0.4
// ----START OF FILE----
use macroquad::prelude::{Color,Font, Image, RectOffset, BLACK, BLUE, DARKGRAY, WHITE};
use macroquad::ui::{root_ui, Skin, StyleBuilder};

pub const ARBUTUS_REGULAR_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Arbutus-Regular.ttf");
pub const LATO_REGULAR_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Lato-Regular.ttf");
pub const OPEN_DYSLEXIC: &[u8] = include_bytes!("../assets/fonts/OpenDyslexicMNerdFont-Regular.otf");

// Ścieżka do obrazka przycisku
const BUTTON_NORMAL_IMG_BYTES: &[u8] = include_bytes!("../assets/btn_normal.png");

pub fn create_global_skin() -> Skin {
    let button_image = Image::from_file_with_format(BUTTON_NORMAL_IMG_BYTES, None)
        .map_err(|e| panic!("Failed to load button image: {:?}", e))
        .ok();

    let button_style_builder_base = root_ui()
        .style_builder()
        .font(ARBUTUS_REGULAR_FONT_BYTES)
        .unwrap()
        .font_size(30)
        .text_color(WHITE) // Zmieniono na biały dla lepszego kontrastu z potencjalnie ciemnym przyciskiem
        .background_margin(RectOffset::new(85.0, 89.0, 18.0, 56.0)) // Marginesy 9-slice
        .margin(RectOffset::new(15.0, 15.0, 10.0, 10.0)); // Wewnętrzny padding tekstu, dostosuj

    let button_style = if let Some(img) = button_image.clone() {
        button_style_builder_base
            .background(img.clone()) // Używamy tego samego obrazka dla wszystkich stanów
            .background_hovered(img.clone())
            .background_clicked(img)
            .build()
    } else {
        // Fallback, jeśli obrazek się nie załaduje
        button_style_builder_base
            .color(Color::from_rgba(70, 70, 90, 255))
            .color_hovered(Color::from_rgba(90, 90, 110, 255))
            .color_clicked(Color::from_rgba(50, 50, 70, 255))
            .build()
    };

    let label_style = root_ui()
        .style_builder()
        .font(LATO_REGULAR_FONT_BYTES)
        .unwrap()
        .font_size(20)
        .text_color(DARKGRAY)
        .build();
    
    let window_style = root_ui()
        .style_builder()
        .font(ARBUTUS_REGULAR_FONT_BYTES)
        .unwrap()
        .font_size(24)
        .text_color(BLACK)
        // Można dodać tło dla okna, np. z obrazka 9-slice
        // .background(Image::from_file_with_format(include_bytes!("../../assets/ui/window_bg.png"), None).unwrap())
        // .background_margin(RectOffset::new(10.0, 10.0, 30.0, 10.0)) // Marginesy dla tła okna
        .build();
    
    let editbox_style = root_ui()
        .style_builder()
        .font(LATO_REGULAR_FONT_BYTES)
        .unwrap()
        .font_size(20)
        .text_color(BLACK)
        .color(WHITE)
        .color_selected(Color::from_rgba(180, 200, 255, 255))
        .color_hovered(Color::from_rgba(245, 245, 245, 255))
        .margin(RectOffset::new(5.0,5.0,2.0,2.0))
        .build();

    Skin {
        button_style,
        label_style,
        window_style, 
        editbox_style,
        ..root_ui().default_skin().clone()
    }
}
// ----END OF FILE----
// karolrybak/whacka-molee/whacka-molee-351ea95a23ebe18bf0ffbcd3437412c5de79bebd/src/ui_styles.rs
// version:0.0.4