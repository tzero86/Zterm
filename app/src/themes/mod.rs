pub mod default_themes;
pub mod theme;
pub mod theme_chooser;
pub mod theme_creator;
pub mod theme_creator_body;
pub mod theme_creator_modal;
pub mod theme_deletion_body;
pub mod theme_deletion_modal;

use zterm_core::ui::theme::ZtermTheme;

pub fn onboarding_theme_picker_themes() -> [ZtermTheme; 4] {
    [
        default_themes::phenomenon(),
        default_themes::dark_theme(),
        default_themes::light_theme(),
        default_themes::adeberry(),
    ]
}
