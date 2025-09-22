use eframe::egui;
use rgb_emu::{Cpu, Rom, Gui};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let rom: Rom = Rom::new(path);
    let cpu: Cpu = Cpu::new();
    let options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let gui: Gui = Gui::new(cpu);
    let _ = eframe::run_native(
        rom.get_title(),
        options,
        Box::new(|_| Ok(Box::<Gui>::new(gui))),
    );
}
