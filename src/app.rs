use std::path::{self, PathBuf};
use std::sync::Arc;

use log::error;

use crate::MediaPlayer;

/// Struct representing configured state of `soundboardbot`
pub struct SoundboardApp {
    // List of `SoundCue`s to display buttons for
    sound_cues: Vec<SoundCue>,

    // Sender for sending filepaths to media player thread
    player: MediaPlayer,
}

/// Details of a sound that can be played by `soundboardbot`
#[derive(Default)]
struct SoundCue {
    // A given name for a cue
    label: String,
    // Path to the target sound file
    audio_path: Option<PathBuf>,
    // Path to an image to use as the cue's icon
    _image_path: Option<PathBuf>,
}

impl SoundboardApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let player = MediaPlayer::start_on_default_device().expect("Could not start media player");
        let cues = (1..=9).map(|i| SoundCue {
            label: format!("sound {i}"),
            audio_path: Some(path::absolute(PathBuf::from(format!(
                "./assets/sounds/test/test-tts{i}.m4a"
            ))).expect("Could not get absolute path of file")),
            ..Default::default()
        });
        Self {
            sound_cues: Vec::from_iter(cues),
            player,
        }
    }
}

impl eframe::App for SoundboardApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            made_with_credits(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("soundboardbot");
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                for cue in &self.sound_cues {
                    self.soundboard_cue_button(ui, cue);
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

impl SoundboardApp {
    // Render button that spawns a media player task on click
    fn soundboard_cue_button(&self, ui: &mut egui::Ui, cue: &SoundCue) {
        let cue_button = egui::Button::new(&cue.label).min_size([30.0, 40.0].into());
        if ui.add(cue_button).clicked() {
            if let Some(path) = &cue.audio_path {
                log::debug!(
                    "Attempting to play sound \"{}\" ({})",
                    cue.label,
                    path.to_string_lossy(),
                );
                match self.player.queue_sound(Arc::new(path.clone())) {
                    Ok(_) => (),
                    Err(e) => error!("Playback failed with error {:?}", e),
                }
            }
        }
    }
}

fn made_with_credits(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Made by James P using ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to("awedio", "https://github.com/10buttons/awedio");
        ui.label(".");
    });
}
