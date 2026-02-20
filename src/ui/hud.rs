//! HUD overlay — countdown timer and crosshair.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::game_timer::GameTimer;

/// Renders the countdown timer in a rounded box and a dot crosshair.
pub fn render_hud(
    mut contexts: EguiContexts,
    timer: Res<GameTimer>,
    time: Res<Time>,
) {
    let ctx = contexts.ctx_mut();
    let remaining = timer.remaining.ceil() as u32;
    let ms = ((time.elapsed_secs() * 100.0) as u32) % 100;

    // Red when under 10s, amber otherwise
    let color = if timer.remaining <= 10.0 {
        egui::Color32::from_rgb(255, 20, 20)
    } else {
        egui::Color32::from_rgb(255, 80, 0)
    };

    // Timer box — top right corner
    egui::Area::new("hud".into())
        .fixed_pos(egui::pos2(ctx.screen_rect().right() - 200.0, 20.0))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                .show(ui, |ui| {
                    ui.set_min_width(160.0);
                    ui.label(egui::RichText::new(
                        format!("{:02}:{:02}", remaining, ms)
                    ).monospace().size(48.0).color(color).strong());
                });
        });

    // Crosshair — small white dot at screen center
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