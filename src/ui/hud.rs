//! HUD overlay — health and stamina bars.
//! Phase 1: stub only. Phase 2: wire up to PlayerStats.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Renders the player HUD with health and stamina bars.
/// Phase 2: replace hardcoded values with live PlayerStats query.
pub fn render_hud(
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();

    egui::Area::new("hud".into())
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(0, 255, 70));

            ui.label(egui::RichText::new("HEALTH").monospace().size(14.0));
            ui.add(egui::ProgressBar::new(1.0).fill(egui::Color32::from_rgb(255, 50, 50)));

            ui.add_space(4.0);

            ui.label(egui::RichText::new("STAMINA").monospace().size(14.0));
            ui.add(egui::ProgressBar::new(1.0).fill(egui::Color32::from_rgb(50, 50, 255)));
        });
    // Crosshair
    egui::Area::new("crosshair".into())
        .fixed_pos(egui::pos2(
            ctx.screen_rect().center().x - 2.0,
            ctx.screen_rect().center().y - 2.0,
        ))
        .show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(4.0, 4.0),
                egui::Sense::hover(),
            );
            ui.painter().circle_filled(
                rect.center(),
                2.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 180),
            );
        });
}

