#![cfg(debug_assertions)]

use imgui::*;

const FPS_HISTORY_LEN: usize = 60;

pub struct DebugStats {
    window_open: bool,
    fps_history: [f32; FPS_HISTORY_LEN],
    highest_fps: f32,
}

impl Default for DebugStats {
    fn default() -> Self {
        DebugStats {
            window_open: false,
            fps_history: [0.0; FPS_HISTORY_LEN],
            highest_fps: 0.0,
        }
    }
}

impl DebugStats {
    pub fn insert_fps(&mut self, fps: u16) {
        let mut to_write = fps as f32;
        self.highest_fps = self.highest_fps.max(to_write);

        for i in 0..FPS_HISTORY_LEN {
            let i = FPS_HISTORY_LEN - i - 1;
            let b = self.fps_history[i];
            self.fps_history[i] = to_write;
            to_write = b;
        }
    }

    pub fn fps_latest(&self) -> f32 {
        self.fps_history[FPS_HISTORY_LEN - 1]
    }

    pub fn fps_history(&self) -> &[f32] {
        &self.fps_history
    }

    pub fn imgui_render_stats(&mut self, ui: &Ui) {
        let mut opened = self.window_open;
        if opened {
            ui.window(im_str!("Renderer Stats"))
                .opened(&mut opened)
                .build(|| {
                    ui.plot_lines(im_str!("FPS {}:", self.fps_latest()), &self.fps_history)
                        .graph_size((256.0, 64.0))
                        .scale_min(0.0)
                        .scale_max(self.highest_fps + 10.0)
                        .build();
                });
        }
        self.window_open = opened;
    }

    pub fn toggle(&mut self) {
        self.window_open = !self.window_open;
    }
}
