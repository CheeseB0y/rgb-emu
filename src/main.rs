use eframe::egui;
use rgb_emu::{Cpu, Gui, MemBus, Rom};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let rom: Rom = Rom::new(path);
    let title: String = rom.get_title().clone();
    let cpu: Cpu = Cpu::new(MemBus::new(rom));
    let options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let gui: Gui = Gui::new(cpu);
    let _ = eframe::run_native(
        &title,
        options,
        Box::new(|_| Ok(Box::<Gui>::new(gui))),
    );
}
