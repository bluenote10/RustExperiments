use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy)]
enum Tab {
    Foo,
    Bar,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    tab: Tab,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    clipboard_context: ClipboardContext,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            tab: Tab::Foo,
            value: 2.7,
            clipboard_context: ClipboardContext::new().unwrap(),
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
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            tab,
            value,
            clipboard_context,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /*
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel_menu").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });
        */

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                ui.style_mut().override_text_style = Some(egui::TextStyle::Button);

                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();

                let mut new_selected = *tab;
                if ui
                    .selectable_label(new_selected == Tab::Foo, "Foo")
                    .clicked()
                {
                    new_selected = Tab::Foo;
                }
                if ui
                    .selectable_label(new_selected == Tab::Bar, "Bar")
                    .clicked()
                {
                    new_selected = Tab::Bar;
                }
                *tab = new_selected;
            });
        });

        /*
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

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
        */

        if *tab == Tab::Foo {
            egui::CentralPanel::default().show(ctx, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's

                ui.heading("eframe template");
                ui.hyperlink("https://github.com/emilk/eframe_template");
                ui.add(egui::github_link_file!(
                    "https://github.com/emilk/eframe_template/blob/master/",
                    "Source code."
                ));

                let clipboard = clipboard_context.get_contents().unwrap();
                for _ in 0..100 {
                    ui.label("Here ".to_owned() + &clipboard);
                }

                egui::warn_if_debug_build(ui);
            });
        } else {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
