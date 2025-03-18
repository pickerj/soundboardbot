/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
                  // #[serde(skip)] // This how you opt-out of serialization of a field
pub struct SoundboardApp {
    sound_cues: Vec<SoundCue>,
}

impl Default for SoundboardApp {
    fn default() -> Self {
        Self {
            sound_cues: (1..=9 as i32)
                .map(|i| SoundCue {
                    label: i.to_string(),
                    asset_path: Some(format!("../assets/sounds/test-tts{i}.m4a")),
                })
                .collect(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct SoundCue {
    label: String,
    asset_path: Option<String>,
}

impl Default for SoundCue {
    fn default() -> Self {
        Self {
            label: String::from("[No sound loaded]"),
            asset_path: None,
        }
    }
}

impl SoundboardApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SoundboardApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

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
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                for cue in &self.sound_cues {
                    soundboard_cue_button(ui, cue);
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn soundboard_cue_button(ui: &mut egui::Ui, cue: &SoundCue) {
    if ui.button(&cue.label).clicked() {
        println!("Attempted to play sound \"{}\"", cue.label);
    }
}

fn made_with_credits(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Made by James using ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and more tbd~");
    });
}
