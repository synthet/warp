use warpui::assets::asset_cache::AssetSource;
use warpui::elements::{
    Align, CacheOption, ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, Image,
    MainAxisAlignment, MouseStateHandle, ParentElement, Wrap,
};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle};

use super::SettingsSection;
use super::settings_page::{
    MatchData, PageType, SettingsPageEvent, SettingsPageMeta, SettingsPageViewHandle,
    SettingsWidget,
};
use crate::appearance::Appearance;
use crate::channel::ChannelState;
use crate::themes::theme::ColorScheme;
use crate::util::links;
use crate::workspace::WorkspaceAction;

const DISPLAY_VERSION_FALLBACK: &str = match option_env!("WARP_DISPLAY_VERSION") {
    Some(version) => version,
    None => concat!("v", env!("CARGO_PKG_VERSION")),
};

/// Product name shown under the logo.
const PRODUCT_NAME: &str = "Synth Term";

/// Keeps upstream's copyright next to the fork's, matching the exe version resource and
/// `legal/synth-warp-notice.md`.
const COPYRIGHT_NOTICE: &str = "© 2020-2026 Denver Technologies, Inc. and Synth Term contributors";

/// Both colour schemes use the Synth Term mark, which carries its own plate.
const LOGO_LIGHT_ON_DARK: &str = "bundled/svg/warp-logo-with-light-title.svg";
const LOGO_DARK_ON_LIGHT: &str = "bundled/svg/warp-logo-with-dark-title.svg";

/// Same wording as Synth Code's About window, naming this fork's upstream.
const UPSTREAM_NOTICE: &str =
    "Based on Warp. Not affiliated with or endorsed by Denver Technologies, Inc.";

// Sizes follow Synth Code's About window: 64 px icon, headline, then small
// label/value pairs with muted captions.
const LOGO_SIZE: f32 = 64.;
const HEADLINE_FONT_SIZE: f32 = 20.;
const VALUE_FONT_SIZE: f32 = 14.;
const CAPTION_FONT_SIZE: f32 = 12.;

/// "Synth Term 0.3.0", like Synth Code's "Synth Code 1.17.0": the version from the
/// Version row without its leading `v` and build suffix, so the two always agree.
fn headline_text(version: &str) -> String {
    let short = version.trim_start_matches('v');
    let short = short.split('+').next().unwrap_or(short);
    format!("{PRODUCT_NAME} {short}")
}

/// The third-party license list that deploy and the installers place next to the
/// executable, as a file URL. `None` when this build has none (e.g. `cargo run`).
fn dependency_licenses_url() -> Option<&'static str> {
    static URL: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
    URL.get_or_init(|| {
        let exe = std::env::current_exe().ok()?;
        let path = exe
            .parent()?
            .join("resources")
            .join("THIRD_PARTY_LICENSES.txt");
        path.is_file()
            .then(|| url::Url::from_file_path(&path).ok())
            .flatten()
            .map(String::from)
    })
    .as_deref()
}

pub(crate) fn about_version_text(release_tag: Option<&'static str>) -> &'static str {
    release_tag.unwrap_or(DISPLAY_VERSION_FALLBACK)
}

pub struct AboutPageView {
    page: PageType<Self>,
}

impl AboutPageView {
    pub fn new(_ctx: &mut ViewContext<AboutPageView>) -> Self {
        AboutPageView {
            page: PageType::new_monolith(AboutPageWidget::default(), None, false),
        }
    }
}

impl Entity for AboutPageView {
    type Event = SettingsPageEvent;
}
impl TypedActionView for AboutPageView {
    type Action = ();
}

impl View for AboutPageView {
    fn ui_name() -> &'static str {
        "AboutPage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

#[derive(Default)]
struct AboutPageWidget {
    copy_version_button_mouse_state: MouseStateHandle,
    agpl_license_mouse_state: MouseStateHandle,
    mit_license_mouse_state: MouseStateHandle,
    dependency_licenses_mouse_state: MouseStateHandle,
}

impl SettingsWidget for AboutPageWidget {
    type View = AboutPageView;

    fn search_terms(&self) -> &str {
        "about synth term warp version license agpl mit copyright dependency licenses"
    }

    fn render(
        &self,
        _view: &AboutPageView,
        appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        let theme = appearance.theme();
        let ui_builder = appearance.ui_builder();
        let caption_color = theme.sub_text_color(theme.background()).into_solid();

        let image_path = if theme.inferred_color_scheme() == ColorScheme::LightOnDark {
            LOGO_LIGHT_ON_DARK
        } else {
            LOGO_DARK_ON_LIGHT
        };

        let version = about_version_text(ChannelState::app_version());

        let text = |content: String, font_size: f32, muted: bool| {
            ui_builder
                .span(content)
                .with_soft_wrap()
                .with_style(UiComponentStyles {
                    font_size: Some(font_size),
                    font_color: muted.then_some(caption_color),
                    ..Default::default()
                })
                .build()
                .finish()
        };
        let link = |label: &str, url: &str, mouse_state: MouseStateHandle| {
            ui_builder
                .link(label.into(), Some(url.into()), None, mouse_state)
                .soft_wrap(false)
                .with_style(UiComponentStyles {
                    font_size: Some(VALUE_FONT_SIZE),
                    ..Default::default()
                })
                .build()
                .finish()
        };

        let copy_version_icon = appearance
            .ui_builder()
            .copy_button(16., self.copy_version_button_mouse_state.clone())
            .build()
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::CopyVersion(version));
            })
            .finish();

        let version_row = Wrap::row()
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_children([
                text(version.to_string(), VALUE_FONT_SIZE, false),
                Container::new(copy_version_icon)
                    .with_padding_left(6.)
                    .finish(),
            ])
            .finish();

        let identity = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(
                ConstrainedBox::new(
                    Image::new(
                        AssetSource::Bundled { path: image_path },
                        CacheOption::BySize,
                    )
                    .finish(),
                )
                .with_max_height(LOGO_SIZE)
                .with_max_width(LOGO_SIZE)
                .finish(),
            )
            .with_child(
                Container::new(text(headline_text(version), HEADLINE_FONT_SIZE, false))
                    .with_margin_top(12.)
                    .finish(),
            )
            .with_child(
                Container::new(text("Version".into(), CAPTION_FONT_SIZE, true))
                    .with_margin_top(12.)
                    .finish(),
            )
            .with_child(Container::new(version_row).with_margin_top(4.).finish())
            .finish();

        let license_row = Wrap::row()
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_children([
                text("Licensed under ".into(), VALUE_FONT_SIZE, false),
                link(
                    "AGPL-3.0-only",
                    links::LICENSE_AGPL_URL,
                    self.agpl_license_mouse_state.clone(),
                ),
                text(" · ".into(), VALUE_FONT_SIZE, true),
                link(
                    "MIT (warpui)",
                    links::LICENSE_MIT_URL,
                    self.mit_license_mouse_state.clone(),
                ),
            ])
            .finish();

        let mut legal = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(license_row)
            .with_child(
                Container::new(text(UPSTREAM_NOTICE.into(), CAPTION_FONT_SIZE, true))
                    .with_margin_top(6.)
                    .finish(),
            )
            .with_child(
                Container::new(text(COPYRIGHT_NOTICE.into(), CAPTION_FONT_SIZE, true))
                    .with_margin_top(4.)
                    .finish(),
            );
        if let Some(url) = dependency_licenses_url() {
            legal.add_child(
                Container::new(link(
                    "Dependency Licenses",
                    url,
                    self.dependency_licenses_mouse_state.clone(),
                ))
                .with_margin_top(10.)
                .finish(),
            );
        }

        Align::new(
            Flex::column()
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(identity)
                .with_child(Container::new(legal.finish()).with_margin_top(28.).finish())
                .finish(),
        )
        .finish()
    }
}

impl SettingsPageMeta for AboutPageView {
    fn section() -> SettingsSection {
        SettingsSection::About
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        true
    }

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
        self.page.update_filter(query, ctx)
    }

    fn scroll_to_widget(&mut self, widget_id: &'static str) {
        self.page.scroll_to_widget(widget_id)
    }

    fn clear_highlighted_widget(&mut self) {
        self.page.clear_highlighted_widget();
    }
}

impl From<ViewHandle<AboutPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<AboutPageView>) -> Self {
        SettingsPageViewHandle::About(view_handle)
    }
}

#[cfg(test)]
#[path = "about_page_tests.rs"]
mod tests;
