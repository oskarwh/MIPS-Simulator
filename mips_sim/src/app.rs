/* 
use egui_extras::{Size, StripBuilder, TableBuilder};
use std::collections::hash_map;

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
        (Heading, FontId::new(35.0, Proportional)),
        (heading2(), FontId::new(25.0, Proportional)),
        (heading3(), FontId::new(23.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(18.0, Proportional)),
        (Button, FontId::new(16.0, Proportional)),
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
    labels: hash_map::HashMap<String, u32>,
    registers: hash_map::HashMap<&'static str, u32>,
    instructions: Vec<u32>,
    mips_instructions: Vec<(String, bool)>,
    program_counter: u32,
}

impl MipsApp {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        label_table: hash_map::HashMap<String, u32>,
        register_table: hash_map::HashMap<&'static str, u32>,
        instructions: Vec<u32>,
        mips_instructions: Vec<(String, bool)>,
    ) -> MipsApp {
        configure_text_styles(&cc.egui_ctx);
        let open_file: String;

        MipsApp {
            selected: DataFormat::Hex,
            labels: label_table,
            registers: register_table,
            instructions: instructions,
            mips_instructions: mips_instructions,
            program_counter: 0,
        }
    }

    fn write_int(number: u32, dformat: &DataFormat) -> String {
        match dformat {
            DataFormat::Dec => String::from(format!("{}", number)),
            DataFormat::Hex => String::from(format!("{:#010x}", number)),
            DataFormat::Bin => String::from(format!("{:#032b}", number)),
        }
    }

    fn memory_table(data_format: &DataFormat, ui: &mut egui::Ui) {
        // Setup table
        ui.push_id(1, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::remainder().at_least(150.0))
                .column(Size::remainder().at_least(150.0))
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
                    // Iterate data memory
                    for row_index in 0..999 {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                ui.label(MipsApp::write_int(row_index * 4, data_format));
                            });
                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                ui.label(MipsApp::write_int(0, data_format));
                            });
                        });
                    }
                });
        });
    }

    fn instruction_table(
        data_format: &DataFormat,
        ui: &mut egui::Ui,
        instructions: &Vec<u32>,
        mips_instructions: &Vec<(String, bool)>,
        program_counter: &u32,
    ) {
        ui.push_id(2, |ui| {
            TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .striped(true)
                .column(Size::exact(25 as f32))
                .column(Size::remainder().at_least(100.0))
                .column(Size::remainder().at_least(100.0))
                .column(Size::remainder().at_least(200.0))
                .resizable(false)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.label(RichText::new("").text_style(heading2()).strong());
                    });
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
                    let mut i = 0;
                    // Iterate over listing file, only add rows containing machine code
                    for mips_instruction in mips_instructions.iter() {
                        if mips_instruction.1 {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    // Print arrow for keeping track of where in the code the user is.
                                    if i == *program_counter {
                                        ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                        ui.label(
                                            RichText::new("➡").text_style(heading2()).strong(),
                                        );
                                    }
                                });
                                row.col(|ui| {
                                    ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                    ui.label(MipsApp::write_int(i * 4, data_format));
                                });
                                row.col(|ui| {
                                    ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                    ui.label(MipsApp::write_int(
                                        (*instructions)[i as usize],
                                        data_format,
                                    ));
                                });
                                row.col(|ui| {
                                    ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                    ui.label(mips_instruction.0.clone());
                                });
                            });
                            i += 1;
                        }
                    }
                });
        });
    }

    fn register_table(
        data_format: &DataFormat,
        ui: &mut egui::Ui,
        registers: &hash_map::HashMap<&'static str, u32>,
    ) {
        ui.push_id(3, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::remainder().at_least(75.0))
                .column(Size::remainder().at_most(75.0))
                .column(Size::remainder().at_least(100.0))
                .resizable(false)
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
                    // Transform hashmap into vector so it can be sorted.
                    let mut sorted_reg: Vec<_> = registers.iter().collect();
                    sorted_reg.sort_by(|a, b| a.1.cmp(&b.1));

                    for (name, num) in sorted_reg {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                ui.label(num.to_string());
                            });
                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                ui.label(*name);
                            });
                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                ui.label(MipsApp::write_int(0, data_format));
                            });
                        });
                    }
                });
        });
    }

    fn symbol_table(
        data_format: &DataFormat,
        ui: &mut egui::Ui,
        labels: &hash_map::HashMap<String, u32>,
    ) {
        // Build symbol table
        ui.push_id(4, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Size::remainder().at_least(100.0))
                .column(Size::remainder().at_least(100.0))
                .resizable(false)
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.label(RichText::new("Name").text_style(heading2()).strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Address").text_style(heading2()).strong());
                    });
                })
                .body(|mut body| {
                    for (name, addr) in labels {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                ui.label(name.clone());
                            });
                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                                ui.label(MipsApp::write_int(*addr * 4, data_format));
                            });
                        });
                    }
                });
        });
    }

    fn mips_code(ui: &mut egui::Ui, mips_instructions: &Vec<(String, bool)>) {}
}

impl eframe::App for MipsApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            selected,
            labels,
            registers,
            instructions,
            mips_instructions,
            program_counter,
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
                    // Option for opening file
                    if ui.button("Open File").clicked() {
                        let open_file: String;
                        match tinyfiledialogs::open_file_dialog("Open", "", None) {
                            Some(file) => open_file = file,
                            None => open_file = "null".to_string(),
                        }
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });
        // Create left side panel containing data memory
        egui::SidePanel::left("left_side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Data memory panel");
                ui.vertical(|ui| {
                    MipsApp::memory_table(&selected, ui);
                });
            });

        // Create right panel
        egui::SidePanel::right("right_side_panel")
            .resizable(true)
            .min_width(400 as f32)
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
                            MipsApp::register_table(&selected, ui, registers);
                        });
                        // Add cell for label table
                        strip.cell(|ui| {
                            ui.heading("Label table");
                        });
                        // We add a nested strip in the bottom cell:
                        strip.cell(|ui| {
                            MipsApp::symbol_table(&selected, ui, labels);
                        });
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                ui.heading("Intsruction memory");
                // Add buttons for stepping through the program.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                    ui.heading(format!("PC: {}", *program_counter));
                    if ui.add(egui::Button::new("Reset")).clicked() {
                        // Reset memory temp
                        *program_counter = 0;
                    }
                    // Add Run button
                    if ui.add(egui::Button::new("Run")).clicked() {
                        // Run program
                    }
                    // Add step
                    if ui.add(egui::Button::new("Step")).clicked() {
                        // Step forward in program
                        if *program_counter as usize != instructions.len() - 1 {
                            *program_counter += 1;
                        }
                    }
                });
            });
            StripBuilder::new(ui)
                .size(Size::relative(0.6)) // top cell
                .size(Size::remainder().at_most(30.))
                .size(Size::relative(1.0)) // bottom cell
                .vertical(|mut strip| {
                    // Add the top 'cell'
                    strip.cell(|ui| {
                        MipsApp::instruction_table(
                            &selected,
                            ui,
                            instructions,
                            mips_instructions,
                            program_counter,
                        );
                    });
                    // Add cell for label table
                    strip.cell(|ui| {
                        ui.heading("Input File");
                    });

                    strip.cell(|ui| {
                        MipsApp::symbol_table(&selected, ui, labels);
                    });
                });

            ui.vertical(|ui| {
                MipsApp::instruction_table(
                    &selected,
                    ui,
                    instructions,
                    mips_instructions,
                    program_counter,
                );
            });
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
}*/
