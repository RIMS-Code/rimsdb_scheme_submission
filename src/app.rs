use egui::RichText;
use strum::IntoEnumIterator;

use crate::{Elements, GroundState, Lasers, Transition, TransitionUnit};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    scheme_element: Elements,
    scheme_gs: GroundState,
    scheme_ip_term_symbol: String,
    scheme_lasers: Lasers,
    scheme_transitions: [Transition; 7],
    scheme_unit: TransitionUnit,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            scheme_element: Elements::H,
            scheme_gs: GroundState {
                level: "0".to_owned(),
                term_symbol: String::new(),
            },
            scheme_ip_term_symbol: String::new(),
            scheme_lasers: Lasers::TiSa,
            scheme_transitions: [
                Transition::new_empty(),
                Transition::new_empty(),
                Transition::new_empty(),
                Transition::new_empty(),
                Transition::new_empty(),
                Transition::new_empty(),
                Transition::new_empty(),
            ],
            scheme_unit: TransitionUnit::CM1,
        }
    }
}

impl TemplateApp {
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

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(
                RichText::new("Resonance Ionization Scheme Submission")
                    .strong(),
            );
            ui.add_space(VERTICAL_SPACE);

            ui.label(RichText::new(INTRO_MESSAGE).strong());
            ui.add_space(VERTICAL_SPACE);

            ui.collapsing("Usage", |ui| { ui.label(USAGE_MESSAGE_GENERAL); });
            ui.add_space(VERTICAL_SPACE);

            ui.separator();
            ui.add_space(VERTICAL_SPACE);

            ui.heading(RichText::new("Scheme").strong());
            ui.add_space(VERTICAL_SPACE);

            ui.collapsing("Usage Scheme", |ui| { ui.label(USAGE_MESSAGE_SCHEME); });
            ui.add_space(VERTICAL_SPACE);

            // Element
            ui.horizontal(|ui| {
                ui.label("Element:");
                egui::ComboBox::from_id_source("Element")
                    .selected_text(format!("{:?}", self.scheme_element))
                    .show_ui(ui, |ui| {
                        for element in Elements::iter() {
                            let tmp_label = format!("{:?}", &element).to_owned();
                            ui.selectable_value(&mut self.scheme_element, element, tmp_label);
                        }
                    });
            });
            ui.add_space(VERTICAL_SPACE);

            // Units
            ui.horizontal(|ui| {
                ui.label("Unit:");
                ui.radio_value(&mut self.scheme_unit, TransitionUnit::CM1, TransitionUnit::CM1.to_string());
                ui.radio_value(&mut self.scheme_unit, TransitionUnit::NM, TransitionUnit::NM.to_string());
            });
            ui.add_space(VERTICAL_SPACE);

            // Grid with the scheme
            egui::Grid::new("scheme_grid")
                .min_col_width(COL_MIN_WIDTH)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("");
                    ui.label("Level");
                    ui.label("Term Symbol");
                    ui.label("Transition strength (1/s)");
                    ui.label("Manifold");
                    ui.label("Properties");
                    ui.end_row();

                    ui.label("Ground state (1/cm):");
                    ui.add(egui::TextEdit::singleline(&mut self.scheme_gs.level)
                        .desired_width(TEXT_INPUT_WIDTH)
                        .horizontal_align(egui::Align::RIGHT)
                    );
                    ui.add(egui::TextEdit::singleline(&mut self.scheme_gs.term_symbol)
                        .desired_width(TEXT_INPUT_WIDTH)
                        .horizontal_align(egui::Align::RIGHT)
                    );
                    ui.end_row();

                    for (it, trans) in self.scheme_transitions.iter_mut().enumerate() {
                        let unit: String;
                        if trans.low_lying {
                            unit = "1/cm".to_owned();
                        } else {
                            unit = self.scheme_unit.to_string();
                        }
                        ui.label(format!("Step {} ({}):", it + 1, unit));
                        ui.add(egui::TextEdit::singleline(&mut trans.level)
                            .desired_width(TEXT_INPUT_WIDTH)
                            .horizontal_align(egui::Align::RIGHT)
                        );
                        ui.add(egui::TextEdit::singleline(&mut trans.term_symbol)
                            .desired_width(TEXT_INPUT_WIDTH)
                            .horizontal_align(egui::Align::RIGHT)
                        );
                        ui.add(egui::TextEdit::singleline(&mut trans.transition_strength)
                            .desired_width(TEXT_INPUT_WIDTH)
                            .horizontal_align(egui::Align::RIGHT)
                        );
                        ui.checkbox(&mut trans.low_lying, "Low-lying");
                        ui.checkbox(&mut trans.forbidden, "Forbidden");
                        ui.end_row();
                    };

                    ui.label("IP (1/cm):");
                    ui.label(format!("{0:.3}", self.scheme_element.ip()));
                    ui.add(egui::TextEdit::singleline(&mut self.scheme_ip_term_symbol)
                        .desired_width(TEXT_INPUT_WIDTH)
                        .horizontal_align(egui::Align::RIGHT)
                    );
                    ui.end_row();
                });
            ui.add_space(VERTICAL_SPACE);

            // Lasers
            ui.horizontal(|ui| {
                ui.label("Lasers:");
                egui::ComboBox::from_id_source("Lasers")
                    .selected_text(self.scheme_lasers.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.scheme_lasers, Lasers::TiSa, Lasers::TiSa.to_string());
                        ui.selectable_value(&mut self.scheme_lasers, Lasers::Dye, Lasers::Dye.to_string());
                        ui.selectable_value(&mut self.scheme_lasers, Lasers::Both, Lasers::Both.to_string());
                    });
            });
            ui.add_space(VERTICAL_SPACE);

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Submit via GitHub").clicked() {
                    println!("Submit scheme via GitHub.");
                }
                if ui.button("Submit via E-Mail").clicked() {
                    println!("Submit scheme via e-mail.");
                }
                if ui.add(egui::Button::new("Clear all")).clicked() {
                    *self = Default::default();
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                ui.hyperlink_to("Source code", "https://github.com/RIMS-Code/rimsdb_scheme_submission");
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

// Constants to configure the App:
const INTRO_MESSAGE: &str = "You can use this form to submit a new resonance ionization scheme \
to the database.";

const USAGE_MESSAGE_GENERAL: &str = "If you have a config file that from the RIMSSchemeDrawer \
software, you can upload it first. Then fill out any additional information you want to submit in \
the form below.\n\n\
Alternatively, you can skip the file upload and fill out the form from scratch. \n\n \
Detailed information for each segment are given in the individual sections below.";

const USAGE_MESSAGE_SCHEME: &str = "The scheme is the main part of the submission. \
It should contain at a minimum the element, the ground state, as well as one or more transitions.\n \
First select the units that you would like to use (nm or cm<sup>-1</sup>). Then fill out the \
levels, optional term symbols, transmission strengths (in s<sup>-1</sup>), \
and whether the level is a low-lying level or if the transition is forbidden. \
Finally, select the lasers that were used for this scheme. \
Further information can always be provided in the notes.";

// const USAGE_MESSAGE_SATURATION: &str = "The saturation part of the submission is optional. \
// Please select the units that you would like to use. Then fill out the ...";

const COL_MIN_WIDTH: f32 = 120.0;
const TEXT_INPUT_WIDTH: f32 = f32::INFINITY;
const VERTICAL_SPACE: f32 = 12.0;