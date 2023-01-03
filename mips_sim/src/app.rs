use crate::{simulation_controller::SimulationController, units::unit};
use eframe::epaint::text;
use egui::{
    Align, Button, Color32, ComboBox, FontFamily, FontId, RichText, ScrollArea, Slider, TextStyle,
};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use std::{
    collections::hash_map,
    sync::{Arc, Mutex},
};

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
// Configure text styles.
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

const LIGHT_GREEN: Color32 = Color32::from_rgba_premultiplied(60, 171, 60, 255);
const LIGHT_BLUE: Color32 = Color32::from_rgba_premultiplied(61, 112, 186, 255);
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/// if we add new fields, give them default values when deserializing old state
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MipsApp {
    // All data that needs to be accessed while GUI is running.
    simulation_controller: SimulationController,
    selected: DataFormat,
    labels: hash_map::HashMap<String, u32>,
    register_table: hash_map::HashMap<String, u32>,
    registers: Arc<Mutex<Vec<i32>>>,
    data_memory: Arc<Mutex<Vec<i32>>>,
    instructions: Vec<u32>,
    mips_instructions: Vec<(String, bool)>,
    program_counter: Arc<Mutex<u32>>,
    simulation_paused: Arc<Mutex<bool>>,
    simulation_speed: f32,
    data_index: Arc<Mutex<usize>>,
    register_index: Arc<Mutex<usize>>,
    valid_file: bool,
    initial_startup: bool,
    updated_data: Arc<Mutex<bool>>,
    updated_reg: Arc<Mutex<bool>>,
    stepped: bool,
    exit_found: Arc<Mutex<bool>>,
}

impl MipsApp {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        simulation_controller: SimulationController,
    ) -> MipsApp {
        configure_text_styles(&cc.egui_ctx);

        MipsApp {
            simulation_controller,
            selected: DataFormat::Hex,
            labels: hash_map::HashMap::new(),
            register_table: Self::setup_registers_table(),
            registers: Arc::new(Mutex::new(vec![0; 32])),
            data_memory: Arc::new(Mutex::new(vec![0; unit::MAX_WORDS])),
            instructions: Vec::new(),
            mips_instructions: Vec::new(),
            program_counter: Arc::new(Mutex::new(0)),
            simulation_paused: Arc::new(Mutex::new(true)),
            simulation_speed: 8.0,
            data_index: Arc::new(Mutex::new(1001)),
            register_index: Arc::new(Mutex::new(33)),
            valid_file: true,
            initial_startup: true,
            updated_data: Arc::new(Mutex::new(false)),
            updated_reg: Arc::new(Mutex::new(false)),
            stepped: false,
            exit_found: Arc::new(Mutex::new(false)),
        }
    }

    /// Initilizes the register table which contains all allowed registers.
    ///
    /// # Arguments
    ///
    /// * `registers` - The register table.
    ///
    fn setup_registers_table() -> hash_map::HashMap<String, u32> {
        let mut reg_table = hash_map::HashMap::new();
        reg_table.insert("zero".to_string(), 0);
        reg_table.insert("at".to_string(), 1);
        reg_table.insert("v0".to_string(), 2);
        reg_table.insert("v1".to_string(), 3);

        reg_table.insert("a0".to_string(), 4);
        reg_table.insert("a1".to_string(), 5);
        reg_table.insert("a2".to_string(), 6);
        reg_table.insert("a3".to_string(), 7);

        reg_table.insert("t0".to_string(), 8);
        reg_table.insert("t1".to_string(), 9);
        reg_table.insert("t2".to_string(), 10);
        reg_table.insert("t3".to_string(), 11);
        reg_table.insert("t4".to_string(), 12);
        reg_table.insert("t5".to_string(), 13);
        reg_table.insert("t6".to_string(), 14);
        reg_table.insert("t7".to_string(), 15);

        reg_table.insert("s0".to_string(), 16);
        reg_table.insert("s1".to_string(), 17);
        reg_table.insert("s2".to_string(), 18);
        reg_table.insert("s3".to_string(), 19);
        reg_table.insert("s4".to_string(), 20);
        reg_table.insert("s5".to_string(), 21);
        reg_table.insert("s6".to_string(), 22);
        reg_table.insert("s7".to_string(), 23);

        reg_table.insert("t8".to_string(), 24);
        reg_table.insert("t9".to_string(), 25);
        reg_table.insert("k0".to_string(), 26);
        reg_table.insert("k1".to_string(), 27);
        reg_table.insert("gp".to_string(), 28);
        reg_table.insert("sp".to_string(), 29);
        reg_table.insert("fp".to_string(), 30);
        reg_table.insert("ra".to_string(), 31);
        reg_table
    }

    fn write_i32(number: i32, dformat: &DataFormat) -> String {
        match dformat {
            DataFormat::Dec => String::from(format!("{}", number)),
            DataFormat::Hex => String::from(format!("{:#010x}", number)),
            DataFormat::Bin => String::from(format!("{:#032b}", number)),
        }
    }

    fn write_u32(number: u32, dformat: &DataFormat) -> String {
        match dformat {
            DataFormat::Dec => String::from(format!("{}", number)),
            DataFormat::Hex => String::from(format!("{:#010x}", number)),
            DataFormat::Bin => String::from(format!("{:#032b}", number)),
        }
    }

    fn reset_gui(&mut self) {
        self.data_memory = Arc::new(Mutex::new(vec![0; unit::MAX_WORDS]));
        self.program_counter = Arc::new(Mutex::new(0));
        self.registers = Arc::new(Mutex::new(vec![0; 32]));
        self.data_memory = Arc::new(Mutex::new(vec![0; unit::MAX_WORDS]));
        self.simulation_paused = Arc::new(Mutex::new(true));
        self.data_index = Arc::new(Mutex::new(1001));
        self.register_index = Arc::new(Mutex::new(33));
        self.updated_data = Arc::new(Mutex::new(false));
        self.updated_reg = Arc::new(Mutex::new(false));
    }

    fn memory_table(
        data_format: &DataFormat,
        ui: &mut egui::Ui,
        data_memory: &mut Arc<Mutex<Vec<i32>>>,
        data_index: &mut Arc<Mutex<usize>>,
        updated: &mut Arc<Mutex<bool>>,
    ) {
        // Setup table
        let locked_data_index = *data_index.lock().unwrap();
        let locked_data_memory = data_memory.lock().unwrap();
        ui.push_id(1, |ui| {
            // Create table builder
            let mut tbb = TableBuilder::new(ui);
            // Configure settings
            tbb = tbb
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::remainder().resizable(false))
                .resizable(true);
            // Scroll to row that was updated.
            if *updated.lock().unwrap() {
                tbb = tbb.scroll_to_row(locked_data_index, Some(Align::Center));
                *updated.lock().unwrap() = false;
            }
            // Add header
            tbb.header(30.0, |mut header| {
                header.col(|ui| {
                    ui.label(RichText::new("Address").text_style(heading2()).strong());
                });
                header.col(|ui| {
                    ui.label(RichText::new("Data").text_style(heading2()).strong());
                });
            })
            .body(|body| {
                // Iterate data memory
                body.rows(30.0, locked_data_memory.len(), |row_index, mut row| {
                    let mut text_color = Some(Color32::WHITE);
                    // Change color of row changed.
                    if row_index == locked_data_index {
                        text_color = Some(LIGHT_GREEN);
                    }
                    row.col(|ui| {
                        ui.visuals_mut().override_text_color = text_color;
                        ui.label(MipsApp::write_u32(row_index as u32 * 4, data_format));
                    });
                    row.col(|ui| {
                        ui.visuals_mut().override_text_color = text_color;
                        ui.label(MipsApp::write_i32(
                            locked_data_memory[row_index],
                            data_format,
                        ));
                    });
                })
            });
        });
    }

    fn instruction_table(
        data_format: &DataFormat,
        ui: &mut egui::Ui,
        instructions: &Vec<u32>,
        mips_instructions: &Vec<(String, bool)>,
        program_counter: &Arc<Mutex<u32>>,
        stepped: &mut bool,
    ) {
        let locked_program_counter = *program_counter.lock().unwrap();
        ui.push_id(2, |ui| {
            let mut tbb = TableBuilder::new(ui);
            tbb = tbb
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .striped(true)
                .column(Column::remainder().at_most(25.0).resizable(false))
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .resizable(true);
            // If data was updated, scroll to row.
            if *stepped {
                tbb = tbb.scroll_to_row(locked_program_counter as usize, Some(Align::Center));
                *stepped = false;
            }
            tbb.header(30.0, |mut header| {
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
                // Iterate over listing file, only add rows containing machine code
                let mut machine_index = 0;
                for (line, contains_code) in mips_instructions {
                    let mut text_color = Some(Color32::WHITE);
                    // Only add row to table if line contains machine code.
                    if *contains_code {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                // Print arrow for keeping track of where in the code the user is.
                                if machine_index == locked_program_counter {
                                    ui.visuals_mut().override_text_color = text_color;
                                    ui.label(RichText::new("➡").text_style(heading2()).strong());
                                    text_color = Some(LIGHT_BLUE);
                                }
                            });

                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = text_color;
                                ui.label(MipsApp::write_u32(machine_index * 4, data_format));
                            });

                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = text_color;
                                ui.label(MipsApp::write_u32(
                                    instructions[machine_index as usize],
                                    data_format,
                                ));
                            });

                            row.col(|ui| {
                                ui.visuals_mut().override_text_color = text_color;
                                ui.label(line.clone().trim());
                            });
                        });
                        machine_index += 1;
                    }
                }
            });
        });
    }

    fn register_table(
        data_format: &DataFormat,
        ui: &mut egui::Ui,
        register_table: &hash_map::HashMap<String, u32>,
        registers: &mut Arc<Mutex<Vec<i32>>>,
        register_index: &mut Arc<Mutex<usize>>,
        updated: &mut Arc<Mutex<bool>>,
    ) {
        // If data was updated, scroll to row. Ugly solution, I know.
        // Lock registers & reg index
        let locked_reg_index = *register_index.lock().unwrap();
        let locked_registers = registers.lock().unwrap();
        ui.push_id(3, |ui| {
            let mut tbb = TableBuilder::new(ui);
            tbb = tbb
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .resizable(true);
            // If data was updated, scroll to row.
            if *updated.lock().unwrap() {
                println!("Jump to row: {}", locked_reg_index);
                tbb = tbb.scroll_to_row(locked_reg_index, Some(Align::Center));
                *updated.lock().unwrap() = false;
            }
            tbb.header(30.0, |mut header| {
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
                let mut sorted_reg: Vec<_> = register_table.iter().collect();
                sorted_reg.sort_by(|a, b| a.1.cmp(&b.1));

                let mut i = 0;
                for (name, num) in sorted_reg {
                    body.row(30.0, |mut row| {
                        let mut text_color = Some(Color32::WHITE);
                        // Change color of row changed.
                        if i == locked_reg_index {
                            text_color = Some(LIGHT_GREEN);
                        }
                        row.col(|ui| {
                            ui.visuals_mut().override_text_color = text_color;
                            ui.label(num.to_string());
                        });
                        row.col(|ui| {
                            ui.visuals_mut().override_text_color = text_color;
                            ui.label(name);
                        });
                        row.col(|ui| {
                            ui.visuals_mut().override_text_color = text_color;
                            ui.label(MipsApp::write_i32(locked_registers[i], data_format));
                        });
                    });
                    i += 1;
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
                .column(Column::remainder())
                .column(Column::remainder())
                .resizable(true)
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
                                ui.label(MipsApp::write_u32(*addr * 4, data_format));
                            });
                        });
                    }
                });
        });
    }

    fn mips_code(
        ui: &mut egui::Ui,
        mips_instructions: &Vec<(String, bool)>,
        program_counter: &Arc<Mutex<u32>>,
        stepped: &mut bool,
    ) {
        let locked_program_counter = *program_counter.lock().unwrap();
        ui.push_id(5, |ui| {
            let mut tbb = TableBuilder::new(ui);
            tbb = tbb
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::remainder())
                .resizable(false)
                .striped(true);
            // If data was updated, scroll to row.
            if *stepped {
                tbb = tbb.scroll_to_row(locked_program_counter as usize, Some(Align::Center));
                *stepped = false;
            }

            tbb.header(30.0, |mut header| {
                header.col(|ui| {
                    ui.label(RichText::new("").text_style(heading2()).strong());
                });
                header.col(|ui| {
                    ui.label(RichText::new("").text_style(heading2()).strong());
                });
            })
            .body(|mut body| {
                // Iterate over listing file, only add rows containing machine code
                let mut machine_index = 0;
                for (line, contains_code) in mips_instructions {
                    let mut text_color = Some(Color32::WHITE);
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            // Only print arrow if line contains instruction.
                            if *contains_code {
                                // Print arrow for keeping track of where in the code the user is.
                                if machine_index as u32 == locked_program_counter {
                                    ui.visuals_mut().override_text_color = text_color;
                                    ui.label(RichText::new("➡").text_style(heading2()).strong());
                                    // Change textcolor
                                    text_color = Some(LIGHT_GREEN);
                                }
                                machine_index += 1;
                            }
                        });
                        row.col(|ui| {
                            ui.visuals_mut().override_text_color = text_color;
                            ui.label(line.clone());
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
        // Do not show processor interface until file provided is valid.
        if self.initial_startup {
            egui::Window::new("MIPS Simulator")
                .resizable(true)
                .show(ctx, |ui| {
                    //ui.spacing_mut().window_margin = Margin::same(20.0);
                    ui.visuals_mut().override_text_color = Some(LIGHT_GREEN);
                    ui.heading("Welcome!");
                    ui.group(|ui| {
                        ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                        ui.label(GREETING_MESSAGE);
                    });
                    // Add a bit of spacing.
                    ui.add_space(10.0);
                    // Button for selecting assembly file
                    ui.horizontal_centered(|ui| {
                        ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                        if ui.button("Select File").clicked() {
                            let open_file: String;
                            match tinyfiledialogs::open_file_dialog("Open", "", None) {
                                Some(file) => open_file = file,
                                None => open_file = "null".to_string(),
                            }

                            if let Some((machine_code, assembler_code, labels, contains_errors)) =
                                self.simulation_controller.setup_simulation(&open_file)
                            {
                                self.initial_startup = false;
                                self.instructions = machine_code;
                                self.mips_instructions = assembler_code;
                                self.labels = labels;
                                // Is valid if file contains no errors.
                                self.valid_file = !contains_errors;
                            } else {
                                // File is invalid
                                self.valid_file = false;
                            }
                        }

                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                });
        } else {
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
                            if let Some((machine_code, assembler_code, labels, contains_error)) =
                                self.simulation_controller.setup_simulation(&open_file)
                            {
                                self.instructions = machine_code;
                                self.mips_instructions = assembler_code;
                                self.labels = labels;
                                // Reset tables
                                self.valid_file = !contains_error;
                                MipsApp::reset_gui(self);
                            } else {
                                //FILE NOT FOUND WADDAFUCK
                            }
                        }
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                });
            });
            // Check if file is valid.
            if self.valid_file {
                // Create left side panel containing data memory
                egui::SidePanel::left("left_side_panel")
                    .resizable(true)
                    .width_range(300.0..=600.0)
                    .show(ctx, |ui| {
                        ui.heading("Data memory");
                        ui.vertical(|ui| {
                            ScrollArea::horizontal()
                                .id_source("data scroll")
                                .show(ui, |ui| {
                                    MipsApp::memory_table(
                                        &mut self.selected,
                                        ui,
                                        &mut self.data_memory,
                                        &mut self.data_index,
                                        &mut self.updated_data,
                                    );
                                });
                        });
                    });

                // Create right panel
                egui::SidePanel::right("right_side_panel")
                    .resizable(true)
                    .width_range(200.0..=500.0)
                    .show(ctx, |ui| {
                        // The right panel holding information about registers.
                        ui.horizontal(|ui| {
                            ComboBox::from_label("Data format")
                                .selected_text(format!("{:?}", self.selected))
                                .show_ui(ui, |ui| {
                                    // TODO: Add to so it actually impacts format of values.
                                    ui.selectable_value(&mut self.selected, DataFormat::Hex, "Hex");
                                    ui.selectable_value(
                                        &mut self.selected,
                                        DataFormat::Dec,
                                        "Decimal",
                                    );
                                    ui.selectable_value(
                                        &mut self.selected,
                                        DataFormat::Bin,
                                        "Binary",
                                    );
                                });
                        });

                        ScrollArea::horizontal()
                            .id_source("Register scroll")
                            .show(ui, |ui| {
                                MipsApp::register_table(
                                    &self.selected,
                                    ui,
                                    &mut self.register_table,
                                    &mut self.registers,
                                    &mut self.register_index,
                                    &mut self.updated_reg,
                                );
                            });
                    });

                // Add Central panel
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The central panel the region left after adding TopPanel's and SidePanel's
                    ui.horizontal(|ui| {
                        ui.heading("Instruction memory");
                        // Add buttons for stepping through the program.
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.visuals_mut().override_text_color = Some(Color32::WHITE);
                            {
                                let locked_program_counter = *self.program_counter.lock().unwrap();
                                ui.heading(format!("PC: {}", locked_program_counter));
                            }

                            if ui
                                .add_enabled(
                                    *self.simulation_paused.lock().unwrap(),
                                    Button::new("Reset"),
                                )
                                .clicked()
                            {
                                // Reset simulation & GUI
                                self.simulation_controller
                                    .reset_simulation(&mut self.instructions);
                                MipsApp::reset_gui(self);
                            }
                            // Add run/pause button depending on the state of the simulation.
                            if *self.simulation_paused.lock().unwrap() {
                                // Simulation is not running, add "Run" button
                                if ui.add(Button::new("Run")).clicked() {
                                    // Run program
                                    *self.simulation_paused.lock().unwrap() = false;
                                    self.simulation_controller.run_program(
                                        self.registers.clone(),
                                        self.data_memory.clone(),
                                        self.program_counter.clone(),
                                        self.simulation_paused.clone(),
                                        self.simulation_speed.clone(),
                                        self.data_index.clone(),
                                        self.register_index.clone(),
                                        self.updated_reg.clone(),
                                        self.updated_data.clone(),
                                        self.exit_found.clone()
                                    );
                                }
                            } else {
                                // Simulation is running, add "Pause" button
                                if ui.add(Button::new("Pause")).clicked() {
                                    // Pause simulation
                                    *self.simulation_paused.lock().unwrap() = true;
                                    self.simulation_controller.pause_simulation();
                                }
                                self.stepped = true;
                            }

                            // Add step button
                            if ui
                                .add_enabled(
                                    *self.simulation_paused.lock().unwrap(),
                                    Button::new("Step"),
                                )
                                .clicked()
                            {
                                *self.simulation_paused.lock().unwrap() = false;
                                self.simulation_controller.step_instruction(
                                    self.registers.clone(),
                                    self.data_memory.clone(),
                                    self.program_counter.clone(),
                                    self.simulation_paused.clone(),
                                    self.data_index.clone(),
                                    self.register_index.clone(),
                                    self.updated_reg.clone(),
                                    self.updated_data.clone(),
                                    self.exit_found.clone()
                                );
                                self.stepped = true;
                                
                            }
                            // Add slider for simulation speed
                            ui.add_enabled(
                                *self.simulation_paused.lock().unwrap(),
                                Slider::new(&mut self.simulation_speed, 0.5..=12.5)
                                    .text("Instructions/Sec"),
                            );
                        });
                    });
                    // Add tables
                    StripBuilder::new(ui)
                        .size(Size::relative(0.6)) // top cell
                        .size(Size::remainder().at_most(30.))
                        .size(Size::remainder()) // bottom cell
                        .vertical(|mut strip| {
                            // Add the top 'cell'
                            strip.cell(|ui| {
                                ScrollArea::horizontal()
                                    .id_source("Instruction scroll")
                                    .show(ui, |ui| {
                                        MipsApp::instruction_table(
                                            &self.selected,
                                            ui,
                                            &self.instructions,
                                            &self.mips_instructions,
                                            &self.program_counter,
                                            &mut self.stepped,
                                        );
                                    });
                            });
                            // Add cell for mips code table & symbol table
                            strip.cell(|ui| {
                                if self.labels.len() != 0 {
                                    // Nestled strip builder
                                    StripBuilder::new(ui)
                                        .size(Size::remainder())
                                        .size(Size::remainder())
                                        .horizontal(|mut strip| {
                                            // Add cell for mips code table
                                            strip.cell(|ui| {
                                                ui.vertical(|ui| {
                                                    // Heading
                                                    ui.heading("Input File");
                                                    // MIPS instruction
                                                    ScrollArea::horizontal()
                                                        .id_source("Mips scroll")
                                                        .show(ui, |ui| {
                                                            MipsApp::mips_code(
                                                                ui,
                                                                &self.mips_instructions,
                                                                &self.program_counter,
                                                                &mut self.stepped,
                                                            );
                                                        });
                                                });
                                            });
                                            // Add cell for symbol code
                                            strip.cell(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.heading("Label Table");
                                                    ScrollArea::horizontal()
                                                        .id_source("Symbol Scroll")
                                                        .show(ui, |ui| {
                                                            // Add symbol table.
                                                            MipsApp::symbol_table(
                                                                &self.selected,
                                                                ui,
                                                                &mut self.labels,
                                                            );
                                                        });
                                                });
                                            });
                                        });
                                } else {
                                    ui.vertical(|ui| {
                                        // Heading
                                        ui.heading("Input File");
                                        // MIPS instruction
                                        ScrollArea::horizontal().id_source("Mips scroll").show(
                                            ui,
                                            |ui| {
                                                MipsApp::mips_code(
                                                    ui,
                                                    &self.mips_instructions,
                                                    &self.program_counter,
                                                    &mut self.stepped,
                                                );
                                            },
                                        );
                                    });
                                }
                            });
                        });
                });
            } else {
                self.initial_startup = false;
                // File is invalid
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The central panel the region left after adding TopPanel's and SidePanel's
                    ui.vertical(|ui| {
                        ui.heading("File invalid");
                        ui.group(|ui| {
                            let mut instruction_file: String = "".to_owned();
                            for (line, _) in self.mips_instructions.clone() {
                                instruction_file = format!("{}\n{}", instruction_file, line);
                            }
                            ui.label(instruction_file);
                        });
                    });
                });
            }
        }
        ctx.request_repaint();
    }
}

const GREETING_MESSAGE: &str = r#"
This is a MIPS emulator that emulates a single cycle MIPS processor. 
It displays intruction memory, data memory, registers, and labels.
You may step through each instruction, run the program or reset it. 
But first, you need to select a valid MIPS assembler file which adhere
to the following criteria:
    • May contain empty lines
    • May contain a comment lines which are defined with ”#” that denotes 
      the first position of a comment. Comments can be placed after an instruction.
    • May contain a line with only a label, where the label must be at position 1 of
      the line and end with ”:”.
    • May contain a line with an instruction, with or without a label at the beginning."));

An instruction consists of several parts, each separated by one or more white-space characters.
• An optional label, at the beginning of the line.
• A MIPS instruction, which is preceded by a single white-space character (a tab or blankspace).
• A number of operands, each separated by a comma
• An optional comment.

The instructions that can be used are a subset of the MIPS instruction set and are the following:
add, sub, and, or, nor, slt, lw, sw, beq, addi, ori, srl, sra, j, jr and nop.
"#;
