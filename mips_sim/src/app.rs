use eframe::egui;
use egui::{FontFamily, FontId, RichText, TextStyle};
use egui_extras::{Size, StripBuilder, TableBuilder};
use tracing_subscriber::{fmt::writer, registry::Data};
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum DataFormat {
    Hex,
    Dec,
    Bin,
}

// Different type of headings.
#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}
// Bunch of text styles.
fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;
    use TextStyle::*;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(30.0, Proportional)),
        (heading2(), FontId::new(25.0, Proportional)),
        (heading3(), FontId::new(23.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(24.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
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
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        Default::default()
    }

    fn write_int(number: u32, dformat: &DataFormat) -> String {
        match dformat {
            DataFormat::Dec => String::from(format!("{}", number)),
            DataFormat::Hex => String::from(format!("{:#010x}", number)),
            DataFormat::Bin => String::from(format!("{:#010b}", number)),
        }
    }

    fn memory_table(data_format: &DataFormat, ui: &mut egui::Ui) {
        ui.push_id(1, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Size::remainder().at_least(200.0))
                .column(Size::remainder().at_least(200.0))
                .resizable(true)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.label(RichText::new("Address").text_style(heading2()).strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Data").text_style(heading2()).strong());
                    });
                })
                .body(|mut body| {
                    for row_index in 0..30 {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(MipsApp::write_int(row_index * 4, data_format));
                            });
                            row.col(|ui| {
                                ui.label(MipsApp::write_int(0, data_format));
                            });
                        });
                    }
                });
        });
    }

    fn instruction_table(data_format: &DataFormat, ui: &mut egui::Ui) {
        ui.push_id(2, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Size::remainder().at_least(100.0))
                .column(Size::remainder().at_least(100.0))
                .column(Size::remainder().at_least(100.0))
                .resizable(true)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.label(RichText::new("Address").text_style(heading2()).strong());
                    });
                    header.col(|ui| {
                        ui.label(
                            RichText::new("Machine code")
                                .text_style(heading2())
                                .strong(),
                        );
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Instruction").text_style(heading2()).strong());
                    });
                })
                .body(|mut body| {
                    for row_index in 0..500 {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.heading(MipsApp::write_int(row_index * 4, data_format));
                            });
                            row.col(|ui| {
                                ui.heading(MipsApp::write_int(row_index * 3, data_format));
                            });
                            row.col(|ui| {
                                ui.heading(MipsApp::write_int(0, data_format));
                            });
                        });
                    }
                });
        });
    }

    fn register_table(data_format: &DataFormat, ui: &mut egui::Ui) {
        ui.push_id(3, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Size::remainder().at_least(100.0))
                .column(Size::remainder().at_least(100.0))
                .column(Size::remainder().at_least(100.0))
                .resizable(true)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.label(RichText::new("Register").text_style(heading2()).strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Name").text_style(heading2()).strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Data").text_style(heading2()).strong());
                    });
                })
                .body(|mut body| {
                    for row_index in 0..500 {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(MipsApp::write_int(row_index * 4, data_format));
                            });
                            row.col(|ui| {
                                ui.label("$temp");
                            });
                            row.col(|ui| {
                                ui.label(MipsApp::write_int(0, data_format));
                            });
                        });
                    }
                });
        });
    }

    fn symbol_table(data_format: &DataFormat, ui: &mut egui::Ui) {
        ui.push_id(4, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Size::remainder().at_least(200.0))
                .column(Size::remainder().at_least(200.0))
                .resizable(true)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.label(RichText::new("Address").text_style(heading2()).strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Data").text_style(heading2()).strong());
                    });
                })
                .body(|mut body| {
                    for row_index in 0..30 {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(MipsApp::write_int(row_index * 4, data_format));
                            });
                            row.col(|ui| {
                                ui.label(MipsApp::write_int(0, data_format));
                            });
                        });
                    }
                });
        });
    }
}

impl eframe::App for MipsApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            selected,
            label,
            value,
        } = self;

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

        egui::SidePanel::left("left_side_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Data memory panel");
                ui.vertical(|ui| {
                    MipsApp::memory_table(&selected, ui);
                });
            });

        // Create right panel
        egui::SidePanel::right("right_side_panel")
            .resizable(false)
            .show(ctx, |ui| {
                // The right panel holding information about registers.
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("Data format")
                        .selected_text(format!("{:?}", selected))
                        .show_ui(ui, |ui| {
                            // TODO: Add to so it actually impacts format of values.
                            ui.selectable_value(selected, DataFormat::Hex, "Hex");
                            ui.selectable_value(selected, DataFormat::Dec, "Decimal");
                            ui.selectable_value(selected, DataFormat::Bin, "Binary");
                        });
                });

                // Create tables for registers and symbols
                StripBuilder::new(ui)
                    .size(Size::relative(0.5)) // top cell
                    .size(Size::remainder().at_most(30.))
                    .size(Size::remainder().at_least(300.)) // bottom cell
                    .vertical(|mut strip| {
                        // Add the top 'cell'
                        strip.cell(|ui| {
                            MipsApp::register_table(&selected, ui);
                        });
                        // Add cell for label table
                        strip.cell(|ui| {
                            ui.heading("Label table");
                        });
                        // We add a nested strip in the bottom cell:
                        strip.cell(|ui| {
                            MipsApp::symbol_table(&selected, ui);
                        });
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                ui.heading("Intsruction memory");
                // Add buttons for stepping through the program.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.heading("PC: 0");
                    if ui.add(egui::Button::new("Step")).clicked() {
                        // Reset memory
                    }
                    // Add Run button
                    if ui.add(egui::Button::new("Run")).clicked() {
                        // Run program
                    }
                    // Add step
                    if ui.add(egui::Button::new("Reset")).clicked() {
                        // Step forward in program
                    }
                });
            });
            ui.vertical(|ui| {
                MipsApp::instruction_table(&selected, ui);
            });
            /*
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(400.0)) // top cell
                .size(Size::exact(50.0)) // bottom cell
                .vertical(|mut strip| {
                    // Add the top 'cell'
                    strip.cell(|ui| {

                    });
                });
                */
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
