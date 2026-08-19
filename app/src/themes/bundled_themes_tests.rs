use super::*;

#[test]
fn embedded_catalog_contains_41_valid_warp_themes() {
    assert_eq!(EMBEDDED_THEME_CATALOG.len(), 41);

    for definition in EMBEDDED_THEME_CATALOG {
        serde_yaml::from_str::<WarpTheme>(definition.yaml).unwrap_or_else(|error| {
            panic!(
                "embedded theme {}/{} must parse: {error}",
                definition.collection, definition.id
            )
        });
    }
}

#[test]
fn embedded_catalog_registers_unique_themes_with_plain_names() {
    let display_names = EMBEDDED_THEME_CATALOG
        .iter()
        .filter_map(|definition| definition.additional_display_name)
        .collect::<Vec<_>>();

    assert_eq!(
        display_names,
        [
            "Catppuccin Mocha",
            "GitHub Dark Default",
            "JetBrains Darcula",
            "Nord",
            "Tokyo Night",
            "Ayu Dark",
            "Ayu Light",
            "Ayu Mirage",
            "Gruvbox Dark Hard",
            "Gruvbox Dark Soft",
            "Gruvbox Light Hard",
            "Gruvbox Light Soft",
            "One Dark",
            "One Light",
        ]
    );
    assert_eq!(additional_bundled_themes().count(), display_names.len());
    assert_eq!(display_names.len(), 14);
}

#[test]
fn registered_bundled_display_names_match_yaml_names() {
    for definition in EMBEDDED_THEME_CATALOG {
        let Some(display_name) = definition.additional_display_name else {
            continue;
        };
        let theme = serde_yaml::from_str::<WarpTheme>(definition.yaml).unwrap_or_else(|error| {
            panic!(
                "embedded theme {}/{} must parse: {error}",
                definition.collection, definition.id
            )
        });

        assert_eq!(theme.name().as_deref(), Some(display_name));
    }
}

#[test]
fn packaging_scripts_include_bundled_theme_notices() {
    let unix_script = include_str!("../../../script/prepare_bundled_resources");
    let windows_script = include_str!("../../../script/windows/prepare_bundled_resources.ps1");

    for notice in ["LICENSE-MIT", "LICENSE-APACHE-2.0"] {
        assert!(
            unix_script.contains(&format!("app/assets/bundled/themes/{notice}")),
            "Unix packaging script must include {notice}"
        );
        assert!(
            windows_script.contains(&format!("app\\assets\\bundled\\themes\\{notice}")),
            "Windows packaging script must include {notice}"
        );
    }
}
