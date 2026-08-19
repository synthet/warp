use warp_core::ui::theme::WarpTheme;

use super::theme::BundledTheme;

pub(super) struct EmbeddedThemeDefinition {
    pub collection: &'static str,
    pub id: &'static str,
    pub yaml: &'static str,
    pub additional_display_name: Option<&'static str>,
}

macro_rules! embedded_theme {
    ($collection:literal, $id:literal) => {
        EmbeddedThemeDefinition {
            collection: $collection,
            id: $id,
            yaml: include_str!(concat!(
                "../../assets/bundled/themes/",
                $collection,
                "/",
                $id,
                ".yaml"
            )),
            additional_display_name: None,
        }
    };
    ($collection:literal, $id:literal, $display_name:literal) => {
        EmbeddedThemeDefinition {
            collection: $collection,
            id: $id,
            yaml: include_str!(concat!(
                "../../assets/bundled/themes/",
                $collection,
                "/",
                $id,
                ".yaml"
            )),
            additional_display_name: Some($display_name),
        }
    };
}

pub(super) const EMBEDDED_THEME_CATALOG: &[EmbeddedThemeDefinition] = &[
    embedded_theme!("popular-dark", "catppuccin-mocha", "Catppuccin Mocha"),
    embedded_theme!("popular-dark", "dracula"),
    embedded_theme!("popular-dark", "github-dark-default", "GitHub Dark Default"),
    embedded_theme!("popular-dark", "jetbrains-darcula", "JetBrains Darcula"),
    embedded_theme!("popular-dark", "nord", "Nord"),
    embedded_theme!("popular-dark", "solarized-dark"),
    embedded_theme!("popular-dark", "tokyo-night", "Tokyo Night"),
    embedded_theme!("warp-defaults", "adeberry"),
    embedded_theme!("warp-defaults", "cyber-wave"),
    embedded_theme!("warp-defaults", "dark"),
    embedded_theme!("warp-defaults", "dark-city"),
    embedded_theme!("warp-defaults", "dracula"),
    embedded_theme!("warp-defaults", "fancy-dracula"),
    embedded_theme!("warp-defaults", "gruvbox-dark"),
    embedded_theme!("warp-defaults", "gruvbox-light"),
    embedded_theme!("warp-defaults", "jellyfish"),
    embedded_theme!("warp-defaults", "koi"),
    embedded_theme!("warp-defaults", "leafy"),
    embedded_theme!("warp-defaults", "light"),
    embedded_theme!("warp-defaults", "marble"),
    embedded_theme!("warp-defaults", "phenomenon"),
    embedded_theme!("warp-defaults", "pink-city"),
    embedded_theme!("warp-defaults", "received-referral-reward"),
    embedded_theme!("warp-defaults", "red-rock"),
    embedded_theme!("warp-defaults", "sent-referral-reward"),
    embedded_theme!("warp-defaults", "snowy"),
    embedded_theme!("warp-defaults", "solar-flare"),
    embedded_theme!("warp-defaults", "solarized-dark"),
    embedded_theme!("warp-defaults", "solarized-light"),
    embedded_theme!("warp-defaults", "willow-dream"),
    embedded_theme!("zed-defaults", "ayu-dark", "Ayu Dark"),
    embedded_theme!("zed-defaults", "ayu-light", "Ayu Light"),
    embedded_theme!("zed-defaults", "ayu-mirage", "Ayu Mirage"),
    embedded_theme!("zed-defaults", "gruvbox-dark"),
    embedded_theme!("zed-defaults", "gruvbox-dark-hard", "Gruvbox Dark Hard"),
    embedded_theme!("zed-defaults", "gruvbox-dark-soft", "Gruvbox Dark Soft"),
    embedded_theme!("zed-defaults", "gruvbox-light"),
    embedded_theme!("zed-defaults", "gruvbox-light-hard", "Gruvbox Light Hard"),
    embedded_theme!("zed-defaults", "gruvbox-light-soft", "Gruvbox Light Soft"),
    embedded_theme!("zed-defaults", "one-dark", "One Dark"),
    embedded_theme!("zed-defaults", "one-light", "One Light"),
];

pub(super) fn additional_bundled_themes() -> impl Iterator<Item = (BundledTheme, WarpTheme)> {
    EMBEDDED_THEME_CATALOG.iter().filter_map(|definition| {
        let display_name = definition.additional_display_name?;
        let theme = serde_yaml::from_str(definition.yaml).unwrap_or_else(|error| {
            panic!(
                "embedded theme {}/{} must be valid Warp YAML: {error}",
                definition.collection, definition.id
            )
        });
        Some((
            BundledTheme::new(definition.collection, definition.id, display_name),
            theme,
        ))
    })
}

#[cfg(test)]
#[path = "bundled_themes_tests.rs"]
mod tests;
