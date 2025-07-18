use glam::Vec2;
use imgui::{
    Condition as ImCond,
    TabItemFlags,
    Ui
};
use std::{
    fmt::Debug,
    time::Instant
};
pub trait InspectorPanel : Debug {
    fn get_panel_name(&self) -> &'static str;
    fn draw(&mut self, ui: &Ui) {
        if let Some(_) = ui.tab_item_with_flags(self.get_panel_name(), Some(&mut true), TabItemFlags::empty()) {
            self.draw_contents(ui);
        }
    }
    fn draw_contents(&mut self, ui: &Ui);
}

pub trait InspectorPanelV2: InspectorPanel {
    fn show_panel(&self) -> bool;
}

pub trait BasicPanel {
    fn draw(&mut self, ui: &Ui);
}

const DEFAULT_SIZE: Vec2 = Vec2::new(100., 200.);
const DEFAULT_POSITION: Vec2 = Vec2::new(30., 30.);

pub trait InspectorWindow : Debug {
    fn get_name(&self) -> &str;
    fn get_open_state(&mut self) -> &mut bool;
    fn get_default_size(&self) -> Vec2 { DEFAULT_SIZE }
    fn get_default_position(&self) -> Vec2 { DEFAULT_POSITION }
    fn draw(&mut self, ui: &Ui) {
        let size: [f32; 2] = self.get_default_size().into();
        let pos: [f32; 2] = self.get_default_position().into();
        let self_into = unsafe { &mut *(&raw mut *self) };
        if let Some(_) = ui.window(self.get_name())
            .size(size, ImCond::FirstUseEver)
            .position(pos, ImCond::FirstUseEver)
            .opened(self_into.get_open_state())
            .begin() {
            self.draw_contents(ui);
        }
    }
    fn draw_contents(&mut self, ui: &Ui);
}

#[derive(Debug)]
pub struct AppTime {
    time_elapsed: f32,
    delta: f32,
    frame_count: usize,
    time: Instant
}

impl AppTime {
    pub fn new() -> Self {
        Self {
            time_elapsed: 0.,
            delta: 0.,
            frame_count: 0,
            time: Instant::now()
        }
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
        let now = Instant::now();
        let since = now.duration_since(self.time);
        self.delta = (since.as_micros() as f32) / 1e6;
        self.time_elapsed += self.delta;
        self.time = now;
    }

    pub fn get_time_elapsed(&self) -> f32 { self.time_elapsed }
    pub fn get_delta(&self) -> f32 { self.delta }
    pub fn get_frame_count(&self) -> usize { self.frame_count }
}

pub trait InspectorWindowV2<TState>: Debug {
    fn get_name(&self) -> &str;
    fn get_open_state(&mut self) -> &mut bool;
    fn get_default_size(&self) -> Vec2 { DEFAULT_SIZE }
    fn get_default_position(&self) -> Vec2 { DEFAULT_POSITION }
    fn draw(&mut self, state: &mut TState, ui: &Ui, time: &AppTime) {
        let size: [f32; 2] = self.get_default_size().into();
        let pos: [f32; 2] = self.get_default_position().into();
        let self_into = unsafe { &mut *(&raw mut *self) };
        if let Some(_) = ui.window(self.get_name())
            .size(size, ImCond::FirstUseEver)
            .position(pos, ImCond::FirstUseEver)
            .opened(self_into.get_open_state())
            .begin() {
            self.draw_contents(state, ui, time);
        }
    }
    fn draw_contents(&mut self, app: &mut TState, ui: &Ui, time: &AppTime);
}