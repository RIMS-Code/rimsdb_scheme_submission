use egui::RichText;
use std::future::Future;
use std::sync::mpsc::{channel, Receiver, Sender};
use strum::IntoEnumIterator;

use crate::{
    create_email_link, create_gh_issue, create_json_output, is_doi, load_config_file, Elements,
    GroundState, Lasers, ReferenceEntry, SaturationCurve, SaturationCurveUnit, Transition,
    TransitionUnit,
};

/// We derive Deserialize/Serialize to persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    pub notes: String,
    pub references: Vec<ReferenceEntry>,
    pub saturation_curves: Vec<SaturationCurve>,
    pub scheme_element: Elements,
    pub scheme_gs: GroundState,
    pub scheme_ip_term_symbol: String,
    pub scheme_lasers: Lasers,
    pub scheme_transitions: [Transition; 7],
    pub scheme_last_step_to_ip: bool,
    pub scheme_unit: TransitionUnit,
    pub submitted_by: String,
    #[serde(skip)]
    pub rimsschemedrawer_in: String,
    #[serde(skip)]
    sat_tmp_title: String,
    #[serde(skip)]
    sat_tmp_notes: String,
    #[serde(skip)]
    sat_tmp_unit: SaturationCurveUnit,
    #[serde(skip)]
    sat_tmp_fit: bool,
    #[serde(skip)]
    sat_tmp_xdat: String,
    #[serde(skip)]
    sat_tmp_xdat_unc: String,
    #[serde(skip)]
    sat_tmp_ydat: String,
    #[serde(skip)]
    sat_tmp_ydat_unc: String,
    #[serde(skip)]
    text_channel: (Sender<String>, Receiver<String>),
    #[serde(skip)]
    reference_id: String,
    #[serde(skip)]
    reference_authors: String,
    #[serde(skip)]
    reference_year: String,
    #[serde(skip)]
    error_reference: String,
    #[serde(skip)]
    error_rimsschemedrawer_in: String,
    #[serde(skip)]
    error_saturation: String,
    #[serde(skip)]
    error_submission: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            notes: String::new(),
            references: Vec::new(),
            saturation_curves: Vec::new(),
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
            scheme_last_step_to_ip: false,
            scheme_unit: TransitionUnit::CM1,
            submitted_by: String::new(),
            sat_tmp_title: String::new(),
            sat_tmp_notes: String::new(),
            sat_tmp_unit: SaturationCurveUnit::WCM2,
            sat_tmp_fit: true,
            sat_tmp_xdat: String::new(),
            sat_tmp_xdat_unc: String::new(),
            sat_tmp_ydat: String::new(),
            sat_tmp_ydat_unc: String::new(),
            rimsschemedrawer_in: String::new(),
            text_channel: channel(),
            reference_id: String::new(),
            reference_authors: String::new(),
            reference_year: String::new(),
            error_reference: String::new(),
            error_rimsschemedrawer_in: String::new(),
            error_saturation: String::new(),
            error_submission: String::new(),
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
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // ui.heading("Resonance Ionization Scheme Submission"
                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.heading(RichText::new("Resonance Ionization Scheme Submission").strong());
                ui.add_space(VERTICAL_SPACE);

                ui.label(RichText::new(INTRO_MESSAGE).strong());
                ui.add_space(VERTICAL_SPACE);

                ui.collapsing("Usage", |ui| {
                    ui.label(USAGE_MESSAGE_GENERAL);
                });
                ui.add_space(VERTICAL_SPACE);

                ui.separator();
                ui.add_space(VERTICAL_SPACE);

                ui.heading(RichText::new("Notes").strong());
                ui.add_space(VERTICAL_SPACE);

                ui.text_edit_multiline(&mut self.notes).on_hover_text(
                    "Add any notes for your scheme here. You can use Markdown commands for formatting.",
                );
                ui.add_space(VERTICAL_SPACE);

                ui.separator();
                ui.add_space(VERTICAL_SPACE);

                ui.heading(RichText::new("Scheme").strong());
                ui.add_space(VERTICAL_SPACE);

                ui.collapsing("Usage Scheme", |ui| {
                    ui.label(USAGE_MESSAGE_SCHEME);
                });
                ui.add_space(VERTICAL_SPACE);

                // Upload existing file
                ui.horizontal(|ui| {
                    if ui.button("Load config file")
                        .on_hover_text("Select a RIMSSchemeDrawer file to load.")
                        .clicked() {
                        let sender = self.text_channel.0.clone();
                        let filter = ["json"];
                        let task = rfd::AsyncFileDialog::new()
                            .add_filter("RIMSSchemeDrawer file", &filter)
                            .pick_file();
                        // Context is wrapped in an Arc, so it's cheap to clone as per:
                        // > Context is cheap to clone, and any clones refers to the same mutable data (Context uses refcounting internally).
                        // Taken from https://docs.rs/egui/0.24.1/egui/struct.Context.html
                        let ctx = ui.ctx().clone();
                        execute(async move {
                            let file = task.await;
                            if let Some(file) = file {
                                let text = file.read().await;
                                let _ = sender.send(String::from_utf8_lossy(&text).to_string());
                                ctx.request_repaint();
                            }
                        });
                    }
                    if !self.error_rimsschemedrawer_in.is_empty() {
                        ui.label(
                            RichText::new(&self.error_rimsschemedrawer_in)
                                .color(egui::Color32::RED)
                                .strong(),
                        );
                    }
                });

                // deal with uploaded file
                if let Ok(text) = self.text_channel.1.try_recv() {
                    self.error_rimsschemedrawer_in.clear();
                    self.rimsschemedrawer_in = text;
                    match load_config_file(self) {
                        Ok(_) => (),
                        Err(e) => self.error_rimsschemedrawer_in = e,
                    };
                }

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
                    ui.radio_value(
                        &mut self.scheme_unit,
                        TransitionUnit::CM1,
                        TransitionUnit::CM1.to_string(),
                    );
                    ui.radio_value(
                        &mut self.scheme_unit,
                        TransitionUnit::NM,
                        TransitionUnit::NM.to_string(),
                    );
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
                        ui.label("Transition strength (s¯¹)");
                        ui.label("Manifold");
                        ui.label("Properties");
                        ui.end_row();

                        ui.label("Ground state (cm¯¹):");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.scheme_gs.level)
                                .desired_width(TEXT_INPUT_WIDTH)
                                .horizontal_align(egui::Align::RIGHT),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut self.scheme_gs.term_symbol)
                                .desired_width(TEXT_INPUT_WIDTH)
                                .horizontal_align(egui::Align::RIGHT),
                        );
                        ui.end_row();

                        for (it, trans) in self.scheme_transitions.iter_mut().enumerate() {
                            let unit = match trans.low_lying {
                                true => TransitionUnit::CM1.to_string(),
                                false => self.scheme_unit.to_string(),
                            };
                            let stp_name = match trans.low_lying {
                                true => "Low-lying",
                                false => "Step",
                            };
                            ui.label(format!("{} {} ({}):", stp_name, it + 1, unit));
                            ui.add(
                                egui::TextEdit::singleline(&mut trans.level)
                                    .desired_width(TEXT_INPUT_WIDTH)
                                    .horizontal_align(egui::Align::RIGHT),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut trans.term_symbol)
                                    .desired_width(TEXT_INPUT_WIDTH)
                                    .horizontal_align(egui::Align::RIGHT),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut trans.transition_strength)
                                    .desired_width(TEXT_INPUT_WIDTH)
                                    .horizontal_align(egui::Align::RIGHT),
                            );
                            ui.checkbox(&mut trans.low_lying, "Low-lying");
                            ui.checkbox(&mut trans.forbidden, "Forbidden");
                            ui.end_row();
                        }

                        ui.label("IP (cm¯¹):");
                        ui.label(format!("{0:.3}", self.scheme_element.ip()));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.scheme_ip_term_symbol)
                                .desired_width(TEXT_INPUT_WIDTH)
                                .horizontal_align(egui::Align::RIGHT),
                        );
                        ui.end_row();
                    });
                ui.add_space(VERTICAL_SPACE);

                // Last step to IP
                ui.checkbox(&mut self.scheme_last_step_to_ip, "Draw last step to IP?")
                    .on_hover_text("Check this box if the last step should be drawn to the ionization potential.");
                ui.add_space(VERTICAL_SPACE);

                // Lasers
                ui.horizontal(|ui| {
                    ui.label("Lasers:");
                    egui::ComboBox::from_id_source("Lasers")
                        .selected_text(self.scheme_lasers.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.scheme_lasers,
                                Lasers::TiSa,
                                Lasers::TiSa.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.scheme_lasers,
                                Lasers::Dye,
                                Lasers::Dye.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.scheme_lasers,
                                Lasers::Both,
                                Lasers::Both.to_string(),
                            );
                        });
                });
                ui.add_space(VERTICAL_SPACE);

                ui.separator();
                ui.add_space(VERTICAL_SPACE);

                // SATURATION CURVES
                ui.heading(RichText::new("Saturation curves").strong());
                ui.add_space(VERTICAL_SPACE);

                ui.collapsing("Usage Saturation Curves", |ui| {
                    ui.label(USAGE_MESSAGE_SATURATION);
                });
                ui.add_space(VERTICAL_SPACE);

                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.sat_tmp_title);
                });
                ui.add_space(VERTICAL_SPACE);

                ui.horizontal(|ui| {
                    ui.label("Unit x-axis:");
                    ui.radio_value(
                        &mut self.sat_tmp_unit,
                        SaturationCurveUnit::WCM2,
                        SaturationCurveUnit::WCM2.to_string(),
                    );
                    ui.radio_value(
                        &mut self.sat_tmp_unit,
                        SaturationCurveUnit::W,
                        SaturationCurveUnit::W.to_string(),
                    );
                });
                ui.add_space(VERTICAL_SPACE);

                ui.checkbox(&mut self.sat_tmp_fit, "Fit saturation curve");
                ui.add_space(VERTICAL_SPACE);

                ui.horizontal(|ui| {
                    ui.label("Notes:");
                    ui.text_edit_multiline(&mut self.sat_tmp_notes);
                });
                ui.add_space(VERTICAL_SPACE);

                egui::Grid::new("sat_curve_grid")
                    .min_col_width(COL_MIN_WIDTH)
                    .striped(false)
                    .show(ui, |ui| {
                        let x_dat_name = match self.sat_tmp_unit {
                            SaturationCurveUnit::WCM2 => "Irradiance",
                            SaturationCurveUnit::W => "Power",
                        };
                        ui.label(format!("{} (x-) data", x_dat_name));
                        ui.add(egui::TextEdit::singleline(&mut self.sat_tmp_xdat)
                            .desired_width(TEXT_INPUT_WIDTH));

                        ui.label("x-data uncertainty");
                        ui.add(egui::TextEdit::singleline(&mut self.sat_tmp_xdat_unc)
                            .desired_width(TEXT_INPUT_WIDTH));
                        ui.end_row();

                        ui.label("Signal (y-) data");
                        ui.add(egui::TextEdit::singleline(&mut self.sat_tmp_ydat)
                            .desired_width(TEXT_INPUT_WIDTH));

                        ui.label("y-data uncertainty");
                        ui.add(egui::TextEdit::singleline(&mut self.sat_tmp_ydat_unc)
                            .desired_width(TEXT_INPUT_WIDTH));
                        ui.end_row();
                    });
                ui.add_space(VERTICAL_SPACE);

                ui.horizontal(|ui| {
                    if ui.button("Add or Update")
                        .on_hover_text("Add or update the saturation curve with the given title.")
                        .clicked() {
                        self.error_saturation.clear();

                        if self.sat_tmp_title.is_empty() {
                            self.error_saturation = "Title cannot be empty.".into();
                        }

                        // create a new saturation curve object to push to the vec if no previous error
                        if self.error_saturation.is_empty() {
                            // check if entry already exists
                            let mut index_exists: Option<usize> = None;
                            for (index, entry) in self.saturation_curves.clone().iter().enumerate() {
                                if entry.title.eq(&self.sat_tmp_title) {
                                    index_exists = Some(index);
                                    break;
                                }
                            }
                            match SaturationCurve::new_from_parts(
                                &self.sat_tmp_title,
                                &self.sat_tmp_notes,
                                &self.sat_tmp_unit,
                                self.sat_tmp_fit,
                                &self.sat_tmp_xdat,
                                &self.sat_tmp_xdat_unc,
                                &self.sat_tmp_ydat,
                                &self.sat_tmp_ydat_unc,
                            ) {
                                Ok(sc) => {
                                    match index_exists {
                                        Some(index) => self.saturation_curves[index] = sc,
                                        None => self.saturation_curves.push(sc),
                                    };
                                    self.sat_tmp_title.clear();
                                    self.sat_tmp_notes.clear();
                                    self.sat_tmp_fit = true;
                                    self.sat_tmp_xdat.clear();
                                    self.sat_tmp_xdat_unc.clear();
                                    self.sat_tmp_ydat.clear();
                                    self.sat_tmp_ydat_unc.clear();
                                }
                                Err(err) => self.error_saturation = err,
                            };
                        };
                    }
                    if !self.error_saturation.is_empty() {
                        ui.label(
                            RichText::new(&self.error_saturation)
                                .color(egui::Color32::RED)
                                .strong(),
                        );
                    }
                });
                ui.add_space(VERTICAL_SPACE);

                // Grid view of existing saturation curve
                if !self.saturation_curves.is_empty() {
                    ui.label("Sorted list of existing saturation curve entries:");
                    ui.add_space(VERTICAL_SPACE);

                    egui::Grid::new("saturation_curve_table")
                        .striped(true)
                        .show(ui, |ui| {
                            for (it, val) in self.saturation_curves.clone().iter().enumerate() {
                                ui.label(&val.title);

                                // Move up and down buttons
                                if ui.button("Move up").clicked() && it > 0 {
                                    self.saturation_curves.swap(it, it - 1);
                                }
                                if ui.button("Move down").clicked() && it < self.saturation_curves.len() - 1 {
                                    self.saturation_curves.swap(it, it + 1);
                                }

                                // Edit button
                                if ui.button("Edit entry").clicked() {
                                    self.sat_tmp_title.clone_from(&val.title);
                                    self.sat_tmp_notes.clone_from(&val.notes);
                                    self.sat_tmp_unit = val.units.clone();
                                    self.sat_tmp_fit = val.fit;
                                    self.sat_tmp_xdat = val.get_xdat();
                                    self.sat_tmp_xdat_unc = val.get_xdat_unc();
                                    self.sat_tmp_ydat = val.get_ydat();
                                    self.sat_tmp_ydat_unc = val.get_ydat_unc();
                                }

                                // Delete button
                                if ui.button("Delete").clicked() {
                                    self.saturation_curves.remove(it);
                                }
                                ui.end_row();
                            }
                        });
                    ui.add_space(VERTICAL_SPACE);
                };

                ui.separator();
                ui.add_space(VERTICAL_SPACE);

                ui.heading(RichText::new("References").strong());
                ui.add_space(VERTICAL_SPACE);

                ui.collapsing("Usage References", |ui| {
                    ui.label(USAGE_MESSAGE_REFERENCE);
                });
                ui.add_space(VERTICAL_SPACE);

                egui::Grid::new("reference_entry_grid")
                    .min_col_width(COL_MIN_WIDTH)
                    .striped(false)
                    .show(ui, |ui|{
                        ui.label("Enter DOI or URL to reference:");
                        ui.text_edit_singleline(&mut self.reference_id)
                            .on_hover_text("Enter the DOI only or an URL to the reference here.");
                        ui.end_row();
                        ui.label("Enter author names to be displayed:");
                        ui.text_edit_singleline(&mut self.reference_authors)
                            .on_hover_text("Example: Wendt, Shulaker and Savina, Rothe et al.");
                        ui.end_row();
                        ui.label("Enter year (4 digits):");
                        ui.text_edit_singleline(&mut self.reference_year);
                    });
                ui.add_space(VERTICAL_SPACE);


                if ui
                    .button("Add or Update")
                    .on_hover_text("Add the current reference to the list.")
                    .clicked()
                {
                    let mut index_exists: Option<usize> = None;
                    for (index, entry) in self.references.clone().iter().enumerate() {
                        if entry.id.eq(&self.reference_id) {
                            index_exists = Some(index);
                            break;
                        }
                    }

                    let mut reference_to_write: Option<ReferenceEntry> = None;

                    if !self.reference_id.is_empty() {
                        if self.reference_authors.is_empty() || self.reference_year.is_empty() {  // so we have a doi
                            if is_doi(&self.reference_id) {
                                reference_to_write = Some(ReferenceEntry::new_from_doi(&self.reference_id));
                            } else {
                                self.error_reference = "This does not look like a DOI. If that is intentional, please fill in the Author and Year data.".into();
                            }
                        } else {  // so we have an URL with entries everywhere
                            match self.reference_year.parse::<usize>() {
                                Ok(year) => reference_to_write = Some(ReferenceEntry::new_from_url(&self.reference_id, &self.reference_authors, year)),
                                Err(_) => self.error_reference = "Cannot parse year. Please check it is a number.".into(),
                            };
                        };
                    } else {
                        self.error_reference = "Reference is empty".into();
                    };

                    if let Some(entry) = reference_to_write {
                        match index_exists {
                            Some(index) => self.references[index] = entry,
                            None => self.references.push(entry)
                        };
                        self.reference_id.clear();
                        self.reference_authors.clear();
                        self.reference_year.clear();
                        self.error_reference.clear();
                    }

                };
                ui.add_space(VERTICAL_SPACE);

                ui.label(
                    RichText::new(&self.error_reference)
                        .color(egui::Color32::RED)
                        .strong(),
                );
                ui.add_space(VERTICAL_SPACE);

                if !self.references.is_empty() {
                    ui.label("List of existing references:");
                    ui.add_space(VERTICAL_SPACE);

                    egui::Grid::new("reference_table")
                        .striped(true)
                        .show(ui, |ui| {
                            for (it, val) in self.references.clone().iter().enumerate() {
                                // Label
                                let lbl_hover_text = if val.year == 0 && val.authors.is_empty() {
                                    format!("https://doi.org/{}", val.id)
                                } else {
                                    format!("{} ({})", val.authors, val.year)
                                };
                                ui.label(&val.id).on_hover_text(lbl_hover_text);

                                // Check URL button
                                if ui.button("Open URL").clicked() {
                                    let open_url = egui::OpenUrl {
                                        url: val.get_url(),
                                        new_tab: true,
                                    };
                                    ui.ctx().open_url(open_url);
                                }

                                // Move up and down buttons
                                if ui.button("Move up").clicked() && it > 0 {
                                    self.references.swap(it, it - 1);
                                }
                                if ui.button("Move down").clicked() && it < self.references.len() - 1 {
                                    self.references.swap(it, it + 1);
                                }

                                // Edit button
                                if ui.button("Edit entry").clicked() {
                                    self.reference_id.clone_from(&val.id);
                                    self.reference_authors.clone_from(&val.authors);
                                    self.reference_year = match val.year {
                                        0 => String::new(),
                                        _ => val.year.to_string(),
                                    };
                                }

                                // Delete button
                                if ui.button("Delete").clicked() {
                                    self.references.remove(it);
                                }
                                ui.end_row();
                            }
                        });
                    ui.add_space(VERTICAL_SPACE);
                };

                ui.separator();

                ui.add_space(VERTICAL_SPACE);

                ui.heading(RichText::new("Submitted by").strong());
                ui.add_space(VERTICAL_SPACE);

                ui.text_edit_singleline(&mut self.submitted_by)
                    .on_hover_text("Enter your name here.");
                ui.add_space(VERTICAL_SPACE);

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Submit via GitHub")
                        .on_hover_text("Submit the scheme via GitHub using your account.")
                        .clicked() {
                        self.error_submission.clear();
                        let body = create_json_output(self).unwrap_or_else(|e| {
                            self.error_submission = format!("Error creating JSON output: {}", e);
                            "".to_owned()
                        });
                        let url = create_gh_issue(&body, &self.scheme_element);
                        let open_url = egui::OpenUrl {
                            url,
                            new_tab: true,
                        };
                        if !body.is_empty() {
                            ui.ctx().open_url(open_url);
                        }
                    }
                    if ui.button("Submit via E-Mail")
                        .on_hover_text("Submit the scheme via E-Mail.")
                        .clicked() {
                        self.error_submission.clear();
                        let body = create_json_output(self).unwrap_or_else(|e| {
                            self.error_submission = format!("Error creating JSON output: {}", e);
                            "".to_owned()
                        });
                        let url = create_email_link(&body, &self.scheme_element);
                        let open_url = egui::OpenUrl{
                            url,
                            new_tab: true,
                        };
                        if !body.is_empty() {
                            ui.ctx().open_url(open_url);
                        }
                    }
                    if ui.button("Download configuration").clicked() {
                        self.error_submission.clear();
                        let body = create_json_output(self).unwrap_or_else(|e| {
                            self.error_submission = format!("Error creating JSON output: {}", e);
                            "".to_owned()
                        });
                        if !body.is_empty() {
                            let filter = ["json"];
                            let task = rfd::AsyncFileDialog::new()
                                .set_file_name(format!("{:?}.json", self.scheme_element))
                                .add_filter("JSON file", &filter)
                                .save_file();
                            let contents = body;
                            execute(async move {
                                let file = task.await;
                                if let Some(file) = file {
                                    _ = file.write(contents.as_bytes()).await
                                }
                            });
                        }
                    }

                    // if in debug mode, display a test button
                    if cfg!(debug_assertions) && ui.button("Test").clicked() {
                        self.error_submission.clear();
                        let body = create_json_output(self).unwrap_or_else(|e| {
                            self.error_submission = format!("Error creating JSON output: {}", e);
                            "".to_owned()
                        });
                        println!("{}", body);
                    }

                    if ui.add(egui::Button::new("Clear all")).clicked() {
                        *self = Default::default();
                    }
                    if !self.error_submission.is_empty() {
                        ui.label(
                            RichText::new(&self.error_submission)
                                .color(egui::Color32::RED)
                                .strong(),
                        );
                    }
                });

                ui.add_space(5. * VERTICAL_SPACE);
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    powered_by_egui_and_eframe(ui);
                    ui.hyperlink_to(
                        "Source code",
                        "https://github.com/RIMS-Code/rimsdb_scheme_submission",
                    );
                });
            });
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
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

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

// Constants to configure the App:
const INTRO_MESSAGE: &str = "You can use this form to submit a new resonance ionization scheme \
to the database.";

const USAGE_MESSAGE_GENERAL: &str = "If you have a config file that from the RIMSSchemeDrawer \
software, you can paste and apply it first. Then fill out any additional information you want to submit in \
the form below.\n\
Alternatively, you can skip the file upload and fill out the form from scratch.\n\
Detailed information for each segment are given in the individual sections below.";

const USAGE_MESSAGE_SCHEME: &str = "The scheme is the main part of the submission. \
It should contain at a minimum the element, the ground state, as well as one or more transitions.\n\
First select the units that you would like to use (nm or cm¯¹). Then fill out the \
levels, optional term symbols, transmission strengths (in s¯¹), \
and whether the level is a low-lying level or if the transition is forbidden. \
Finally, select the lasers that were used for this scheme. \
Further information can always be provided in the notes.";

const USAGE_MESSAGE_SATURATION: &str = "To add a saturation curve, you need to add at least \
a title, the unit used for the x-values, x- and y-data. Optionally, you can provide notes and \
uncertainties for the x- and y-data.\n\
Please include the wavelength of the transition and/or a clear identifier which transition is described \
in the title of the submission. \
If you choose units in Watts, please make sure that you include the approximate beam size for the \
laser is included in the notes. \
Finally, data can be pasted, e.g., from Excel, into the individual field. \
Each field needs to contain the same number of values. \
Values can be separated by comma, semicolon, or space.";

const USAGE_MESSAGE_REFERENCE: &str = "You can either provide only a `doi` (leaving the author and year fields empty \
or you can provide a URL to an article as well as an author name and year. \
Please provide the author name in the same way as it should be displayed, i.e., \"Chrysalidis et al.\".";

// const USAGE_MESSAGE_SATURATION: &str = "The saturation part of the submission is optional. \
// Please select the units that you would like to use. Then fill out the ...";

const COL_MIN_WIDTH: f32 = 120.0;
const TEXT_INPUT_WIDTH: f32 = f32::INFINITY;
const VERTICAL_SPACE: f32 = 12.0;
