#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))] 
enum DataFormat {
    Hex,
    Dec,
}
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/// if we add new fields, give them default values when deserializing old state
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))] 
pub struct MipsApp {
    // Example stuff:
    selected: DataFormat,
    label: String,
    // this how you opt-out of serialization of a member
    value: f32,
}

impl Default for MipsApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            selected: DataFormat::Hex,
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl MipsApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        Default::default()
    }
}

impl eframe::App for MipsApp {
    /// Called by the frame work to save state before shutdown.
    //fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //    eframe::set_value(storage, eframe::APP_KEY, self);
    //}

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            selected, 
            label,
             value, } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("left_side_panel").show(ctx, |ui| {
            ui.heading("Data memory panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });
        egui::SidePanel::right("right_side_panel").show(ctx, |ui| {
            // The right panel holding information about registers. 
            ui.horizontal(|ui| {
                ui.label("PC: 0");
                egui::ComboBox::from_label("Data format")
                .selected_text(format!("{:?}", selected))
                .show_ui(ui, |ui| {
                    // TODO: Add to so it actually impacts format of values.
                    ui.selectable_value( selected, DataFormat::Hex, "Hex");
                    ui.selectable_value( selected, DataFormat::Dec, "Decimal");
                });
            });
            // Start adding grid with registers. 
            ui.vertical(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("Register grid").show(ui, |ui| {
                        ui.label("Register");
                        ui.label("Name");
                        ui.label("Data");
                        ui.end_row();
                        for n in 0..10 {
                            // Add register, name and data value.
                            ui.label(format!("{:#010x}", n * 4));
                            ui.label("$temp");
                            ui.label(format!("{:#010x}", 0));
                            ui.end_row();
                        } 
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }

        
    }
}
