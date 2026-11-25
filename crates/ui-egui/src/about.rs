//! Help › About DesignCraft: tabs About · Contributors · Models. About is the splash with the
//! app's name, version and project links; Contributors and Models are the compiled-in credits
//! (`crate::credits`).

use designcraft_engine::links;
use egui::{Color32, Rect, Sense, pos2, vec2};
use serde_json::json;

use crate::DesignApp;
use crate::theme::{Tokens, semibold};

/// The About window's tabs, in `UiState::about_tab` order (`help.about {tab}` names them in lowercase).
pub const ABOUT_TABS: [&str; 3] = ["About", "Contributors", "Models"];

/// Where this build comes from, shown as plain text in the About window.
pub const BASED_ON: &str =
    "Based on DesignCraft by the ArtCraft team (https://github.com/storytold/designcraft). This build is not an ArtCraft product.";

/// The app's name as the About window and the start screen show it, as plain text: DesignCraft
/// (stable) or DesignCraft Nightly.
pub fn app_name(language: &str, channel: crate::channel::Channel) -> String {
    match channel {
        crate::channel::Channel::Stable => crate::i18n::tr(language, "DesignCraft").to_string(),
        crate::channel::Channel::NightlyV1 => channel.name().to_string(),
    }
}

/// A text link that opens `url`. Returns true when clicked.
fn link(ui: &mut egui::Ui, text: &str, url: &str, color: Color32) -> bool {
    let resp = ui.add(egui::Label::new(egui::RichText::new(text).size(13.0).color(color).underline()).sense(Sense::click()));
    let resp = resp.on_hover_text(url).on_hover_cursor(egui::CursorIcon::PointingHand);
    resp.clicked()
}

pub fn show(app: &mut DesignApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let t = Tokens::get(ctx);
    let mut open: Option<&'static str> = None;
    let area = egui::Id::unique("about");
    let instance = area.with("placement");
    let resp = crate::modal_drag::modal(ctx, area, instance).show(ctx, |ui| {
        ui.set_width(if app.ui.about_tab == 0 { 460.0 } else { 820.0_f32.min(ctx.content_rect().width() - 40.0).max(460.0) });
        // The tab row is the title row. The tabs register after the drag area, so they stay clickable.
        let margin = egui::Frame::popup(ui.style()).inner_margin;
        let top = ui.cursor().top();
        let title_row = Rect::from_min_max(
            pos2(ui.max_rect().left() - margin.leftf(), top - margin.topf()),
            pos2(ui.max_rect().right() + margin.rightf(), top + ui.spacing().interact_size.y),
        );
        crate::modal_drag::title_drag(ui, instance, title_row);
        ui.horizontal(|ui| {
            for (i, l) in ABOUT_TABS.iter().enumerate() {
                let i = i as u8;
                if ui.selectable_label(app.ui.about_tab == i, crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, l))).clicked() {
                    app.ui.about_tab = i;
                }
            }
        });
        ui.separator();
        if app.ui.about_tab != 0 {
            let h = 400.0_f32.min(ctx.content_rect().height() * 0.7).max(160.0);
            ui.allocate_ui_with_layout(vec2(ui.available_width(), h), egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_min_height(h);
                if app.ui.about_tab == 1 {
                    crate::credits::contributors_ui(ui, &app.ui.language);
                } else {
                    crate::credits::models_ui(ui, &app.ui.language);
                }
            });
            ui.separator();
            ui.vertical_centered(|ui| {
                if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                    app.ui.about = false;
                }
            });
            return;
        }
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            // The channel's own name and version: DesignCraft (stable) or DesignCraft Nightly.
            let app_name = app_name(&app.ui.language, crate::channel::current());
            ui.label(crate::rtl::widget(ui, egui::RichText::new(app_name).font(semibold(26.0)).color(t.text_strong)));
            crate::rtl::label(
                ui,
                egui::RichText::new(format!("{} {}", crate::i18n::tr(&app.ui.language, "Version"), crate::channel::version()))
                    .size(12.0)
                    .color(t.text_dim),
            );
            // A nightly build says which commit it was made from; the links below include Report an issue.
            if let Some(commit) = crate::channel::commit() {
                let short = commit.get(..12).unwrap_or(commit);
                crate::rtl::label(ui, egui::RichText::new(format!("nightly-v1 · {short}")).size(11.0).color(t.text_dim));
            }
            ui.add_space(4.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Page layout for print and screen — fast, open and scriptable."))
                    .size(13.0)
                    .color(t.text),
            ));
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                // Centre the row of links.
                let w = 160.0;
                ui.add_space(((ui.available_width() - w) / 2.0).max(0.0));
                for (text, key) in [("GitHub", "github"), ("Report an issue", "issues")] {
                    if link(ui, crate::i18n::tr(&app.ui.language, text), links::get(key).unwrap_or_default(), t.accent) {
                        open = Some(key);
                    }
                    ui.add_space(6.0);
                }
            });
            ui.add_space(12.0);
            ui.label(crate::rtl::widget(ui, egui::RichText::new(BASED_ON).size(11.0).color(t.text_dim)));
            ui.add_space(12.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Open source. No telemetry.")).size(11.0).color(t.text_dim),
            ));
            ui.add_space(8.0);
            if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                app.ui.about = false;
            }
        });
    });
    if resp.should_close() {
        app.ui.about = false;
    }
    if let Some(k) = open {
        let _ = app.run(&format!("help.{k}"), json!({}));
    }
}
