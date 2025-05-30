//! Rust port of ImSequencer, part of the ImGuizmo library (MIT License)
//! https://github.com/CedricGuillemet/ImGuizmo/blob/master/ImSequencer.h
//! 
//! The C++ implementation contains a method for controlling the sequencer
//! that acccepts a SequencerInterface class. This contains a set of virtual
//! methods for the application to inherit for their own purposes.
//! While I could technically produce C bindings for this and do some vtable
//! shenanigans, I thought it'd be best to instead port this to Rust.

use bitflags::bitflags;
use glam::Vec2;
use imgui::Ui;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SequencerOptions : u32 {
        const EditNone = 1 << 0;
        const EditStartEnd = 1 << 1;
        const ChangeFrame = 1 << 3;
        const Add = 1 << 4;
        const Delete = 1 << 5;
        const CopyPaste = 1 << 6;
        // const SEQUENCER_EDIT_ALL = Self::SEQUENCER_EDIT_STARTEND | Self::SEQUENCER_CHANGE_FRAME;
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct CustomDraw {
    index: usize,
    custom_rect: [Vec2; 2],
    legend_rect: [Vec2; 2],
    clip_rect: [Vec2; 2],
    legend_clip_rect: [Vec2; 2],
}

impl CustomDraw {

}

#[derive(Debug)]
pub struct SequencerStyle {
    header_color: u32,
    bg_color: u32,
    top_color: u32,
    head_tick_color: u32,
    body_tick_color: u32,
    frame_num_color: u32,
    slot_color: [u32; 2]
}

impl Default for SequencerStyle {
    fn default() -> Self {
        Self {
            header_color: 0xffff0000,
            bg_color: 0xff242424,
            top_color: 0xff3d3837,
            head_tick_color: 0xFF606060,
            body_tick_color: 0x30606060,
            frame_num_color: 0xFFBBBBBB,
            slot_color: [0xff3a3636, 0xff413d3d]
        }
    }
}

impl SequencerStyle {
    pub fn set_header_color(mut self, col: u32) -> Self {
        self.header_color = col;
        self
    }

    pub fn set_bg_color(mut self, col: u32) -> Self {
        self.bg_color = col;
        self
    }

    pub fn set_top_color(mut self, col: u32) -> Self {
        self.top_color = col;
        self
    }

    pub fn set_head_tick_color(mut self, col: u32) -> Self {
        self.head_tick_color = col;
        self
    }

    pub fn set_body_tick_color(mut self, col: u32) -> Self {
        self.body_tick_color = col;
        self
    }

    pub fn set_frame_num_color(mut self, col: u32) -> Self {
        self.frame_num_color = col;
        self
    }

    pub fn set_slot_color(mut self, col: [u32; 2]) -> Self {
        self.slot_color = col;
        self
    }

    fn get_header_color(&self) -> u32 { self.header_color }
    fn get_bg_color(&self) -> u32 { self.bg_color }
    fn get_top_color(&self) -> u32 { self.top_color }
    fn get_head_tick_color(&self) -> u32 { self.head_tick_color }
    fn get_body_tick_color(&self) -> u32 { self.body_tick_color }
    fn get_frame_num_color(&self) -> u32 { self.frame_num_color }
    fn get_slot_color(&self) -> [u32; 2] { self.slot_color }

}

pub trait Sequencer {
    fn get_frame_min(&self) -> u32;
    fn get_frame_max(&self) -> u32;
    fn get_item_count(&self) -> usize;

    fn set_focused(&mut self, focused: bool);
    fn get_focused(&self) -> bool;
    fn get_style(&self) -> &SequencerStyle;
    /* 

    fn begin_edit(&self, index: usize);
    fn end_edit(&self);
    fn get_item_type_count(&self) -> u32 { 0 }
    fn get_item_type_name(&self, index: usize) -> &str { "" }
    fn get_item_label(&self, index: usize) -> &str { "" }

    fn copy(&self);
    fn paste(&self);

    fn get_custom_height(&self, index: usize) -> usize { 0 }
    fn double_click(&self, index: usize);
    */

    fn draw(&mut self, ui: &Ui, flags: SequencerOptions) {
        let draw_list = ui.get_window_draw_list();
        let canvas_pos = Vec2::from(ui.cursor_screen_pos());
        let canvas_size = Vec2::from(ui.content_region_avail());

        let frame_pixel_width = 10.;
        let frame_pixel_width_target = 10.;
        let legend_width = 200;
        let item_height = 20.;
        let frame_count = (self.get_frame_max() - self.get_frame_min()).max(1);
        let control_height = self.get_item_count() as f32 * item_height;
        // zoom in/out
        let visible_frame_count = ((canvas_size.x - legend_width as f32) / frame_pixel_width) as u32;
        let bar_width_ratio = (visible_frame_count as f32 / frame_count as f32).min(1.);
        let bar_width_pixels = bar_width_ratio * (canvas_size.x - legend_width as f32);
        // header
        let header_rect = [ canvas_pos, canvas_pos + Vec2::new(canvas_size.x,  item_height) ];
        let header_size: [f32; 2] = (header_rect[1] - header_rect[0]).into();
        // let scroll_size = Vec2::new(canvas_size.x, 14.);
        if ui.invisible_button("topBar", header_size) {}
        let content_bar_size: [f32; 2] = [ canvas_size.x, control_height ];
        if ui.invisible_button("contentBar", content_bar_size) {}
        self.set_focused(ui.is_window_focused());
        let content_rect = [
            Vec2::from(ui.item_rect_min()),
            Vec2::from(ui.item_rect_max()),
        ];

        draw_list.add_rect(
            Into::<[f32; 2]>::into(header_rect[0]),
            Into::<[f32; 2]>::into(header_rect[1]), 
            self.get_style().get_header_color()).filled(true).build();
        // draw background
        let bg_rect = [canvas_pos, canvas_pos + canvas_size];
        draw_list.add_rect(
            Into::<[f32; 2]>::into(bg_rect[0]), 
            Into::<[f32; 2]>::into(bg_rect[1]), 
            self.get_style().get_bg_color()).filled(true).build();
        // frame top
        let top_rect = [
            Vec2::new(canvas_pos.x + legend_width as f32, canvas_pos.y),
            Vec2::new(canvas_pos.x + canvas_size.x, canvas_pos.y + item_height),
        ];
        draw_list.add_rect(
            Into::<[f32; 2]>::into(top_rect[0]), 
            Into::<[f32; 2]>::into(top_rect[1]), 
            self.get_style().get_top_color()).filled(true).build();
        if flags.contains(SequencerOptions::Add) {

        }
        // frame number and lines
        let mut mod_frame_count = 10;
        let mut frame_step = 1;
        while mod_frame_count * (frame_pixel_width as u32) < 150 {
            mod_frame_count *= 2;
            frame_step *= 2;
        }
        let half_mod_frame_count = mod_frame_count / 2;

        let draw_line = |i: u32, region_height| {
            let base_index = (i % mod_frame_count) == 0 || i == self.get_frame_max() || i == self.get_frame_min();
            let half_index = i % half_mod_frame_count == 0;
            let px = canvas_pos.x + (i as f32 * frame_pixel_width) + legend_width as f32;
            let tick_start = if base_index { 4. } else { if half_index { 10. } else { 14. }};
            let tick_end = if base_index { region_height } else { item_height };
            if px <= canvas_pos.x + canvas_size.x && px >= canvas_pos.x + legend_width as f32 {
                draw_list.add_line([px, canvas_pos.y + tick_start], [px, canvas_pos.y + tick_end], self.get_style().get_head_tick_color()).thickness(1.).build();
                // draw_list.add_line([px, canvas_pos.y + item_height],[px, canvas_pos.y + region_height - 1.],BODY_TICK_COLOR).thickness(1.).build();
            }
            if base_index && px >= canvas_pos.x + legend_width as f32 {
                draw_list.add_text([px + 3., canvas_pos.y], self.get_style().get_frame_num_color(), &format!("{}", i));
            }
        };
        let draw_line_content = |i: u32, item_height: f32| {
            let px = canvas_pos.x + (i as f32 * frame_pixel_width) + legend_width as f32;
            if px <= canvas_pos.x + canvas_size.x && px >= canvas_pos.x + legend_width as f32 {
                draw_list.add_line([px, content_rect[0].y], [px, content_rect[1].y], self.get_style().get_body_tick_color()).thickness(1.).build();
            }
        };

        // draw header ticks
        for i in (self.get_frame_min() / frame_step)..(self.get_frame_max() / frame_step) {
            draw_line(i * frame_step, item_height);
        }
        draw_line(self.get_frame_min(), item_height);
        draw_line(self.get_frame_max(), item_height);
        // draw item names
        for i in 0..self.get_item_count() {
            let text_pos = Vec2::new(content_rect[0].x + 3., content_rect[0].y + i as f32 * item_height + 2.);
            draw_list.add_text(Into::<[f32; 2]>::into(text_pos), 0xffffffff, &format!("label {}", i));
        }
        // slots
        for i in 0..self.get_item_count() {
            let color = self.get_style().get_slot_color()[i & 1];
            let pos = Vec2::new(content_rect[0].x + legend_width as f32, content_rect[0].y + i as f32 * item_height + 1.);
            let size = Vec2::new(canvas_size.x + canvas_pos.x, pos.y + item_height - 1.);
            draw_list.add_rect(
                Into::<[f32; 2]>::into(pos),
                Into::<[f32; 2]>::into(size),
                color
            ).filled(true).build()
        }
        // vertical lines in content
        for i in (self.get_frame_min() / frame_step)..(self.get_frame_max() / frame_step) {
            draw_line_content(i * frame_step, item_height);
        }
        draw_line_content(self.get_frame_min(), item_height);
        draw_line_content(self.get_frame_max(), item_height);
        
    }
}