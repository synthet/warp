use super::*;

#[test]
fn embedded_catalog_contains_43_valid_warp_themes() {
    assert_eq!(EMBEDDED_THEME_CATALOG.len(), 43);

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
fn embedded_catalog_registers_20_additional_themes() {
    assert_eq!(additional_bundled_themes().count(), 20);
}
