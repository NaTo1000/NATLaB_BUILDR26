// NATLaB BUILDR26 — Tauri main entry point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    natlab_buildr26_lib::run();
}
