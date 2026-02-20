//! Game over screen.
//! Phase 1: stub only. Phase 2: wire up to death events.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Renders the game over screen.
pub fn render_game_over(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::BLACK))
        .show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(255, 50, 50));

            ui.vertical_centered(|ui| {
                ui.add_space(200.0);
                ui.label(egui::RichText::new("SYSTEM FAILURE").monospace().size(48.0));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("THE Y2K BUG HAS CONSUMED THE SYSTEM").monospace().size(18.0));
                ui.add_space(40.0);
                ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(0, 255, 70));
                ui.label(egui::RichText::new("[PRESS ENTER TO RESTART]").monospace().size(16.0));
            });

            if keys.just_pressed(KeyCode::Enter) {
                next_state.set(crate::states::GameState::Exploring);
            }
        });
}

/// Renders the win screen.
pub fn render_win(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::BLACK))
        .show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(0, 255, 70));

            ui.vertical_centered(|ui| {
                ui.add_space(200.0);
                ui.label(egui::RichText::new("Y2K BUG PATCHED").monospace().size(48.0));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("SYSTEM RESTORED — HUMANITY SAVED").monospace().size(18.0));
                ui.add_space(40.0);
                ui.label(egui::RichText::new("[PRESS ENTER TO PLAY AGAIN]").monospace().size(16.0));
            });

            if keys.just_pressed(KeyCode::Enter) {
                next_state.set(crate::states::GameState::Exploring);
            }
        });
}