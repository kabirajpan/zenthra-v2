use std::path::{Path, PathBuf};
use std::time::Instant;
use zenthra_core::ObjectFit;
use crate::theme::ThemeMode;

pub struct ViewerState {
    pub images: Vec<PathBuf>,
    pub selected_idx: usize,
    pub last_selected_idx: usize,
    pub zoom: f32,
    pub rotation: f32,
    pub grayscale_val: f32,
    pub fit_mode: ObjectFit,
    pub slideshow_active: bool,
    pub last_slide_time: Instant,
    pub sidebar_visible: bool,
    pub theme: ThemeMode,
    pub show_about: bool,
    // About window state
    pub about_x: f32,
    pub about_y: f32,
    pub window_start_idx: usize,
}

impl ViewerState {
    pub fn new() -> Self {
        // Start with no images — user opens a file/folder explicitly.
        // No platform-specific paths are scanned automatically.
        Self {
            images: Vec::new(),
            selected_idx: 0,
            last_selected_idx: 0,
            zoom: 1.0,
            rotation: 0.0,
            grayscale_val: 0.0,
            fit_mode: ObjectFit::Contain,
            slideshow_active: false,
            last_slide_time: Instant::now(),
            sidebar_visible: true,
            theme: ThemeMode::Dark,
            show_about: true,
            about_x: 390.0,
            about_y: 150.0,
            window_start_idx: 0,
        }
    }

    #[allow(dead_code)]
    pub fn current_image(&self) -> Option<&PathBuf> {
        if self.images.is_empty() {
            None
        } else {
            Some(&self.images[self.selected_idx % self.images.len()])
        }
    }

    pub fn update_slideshow(&mut self) -> bool {
        if self.slideshow_active && !self.images.is_empty() {
            if self.last_slide_time.elapsed().as_secs_f32() >= 3.0 {
                self.selected_idx = (self.selected_idx + 1) % self.images.len();
                self.last_slide_time = Instant::now();
                return true;
            }
        }
        false
    }

    pub fn reset(&mut self) {
        self.zoom = 1.0;
        self.rotation = 0.0;
        self.grayscale_val = 0.0;
        self.fit_mode = ObjectFit::Contain;
        self.slideshow_active = false;
        self.window_start_idx = 0;
    }

    /// Scan a directory and load all supported images from it.
    pub fn load_folder(&mut self, dir_path: &Path) {
        if dir_path.exists() && dir_path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(dir_path) {
                let mut found_images = Vec::new();
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                            let lower = ext.to_lowercase();
                            if lower == "jpg" || lower == "jpeg"
                                || lower == "png" || lower == "webp"
                            {
                                found_images.push(entry_path);
                            }
                        }
                    }
                }
                found_images.sort();
                if !found_images.is_empty() {
                    self.images = found_images;
                    self.selected_idx = 0;
                    self.reset();
                }
            }
        }
    }

    /// Open a single image file; also loads its parent folder so the user
    /// can browse siblings with the filmstrip.
    pub fn load_image_file(&mut self, file_path: &Path) {
        if file_path.exists() && file_path.is_file() {
            if let Some(parent) = file_path.parent() {
                self.load_folder(parent);
                if let Some(idx) = self.images.iter().position(|p| p == file_path) {
                    self.selected_idx = idx;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window_logic() {
        let total_images = 250;
        let window_size: usize = 12;
        let mut window_start_idx: usize = 0;

        // Case 1: selected_idx = 5 is within bounds (1..11), so window_start_idx should remain 0
        let selected_idx: usize = 5;
        let end_idx = window_start_idx + window_size;
        let left_bound = window_start_idx + 1;
        let right_bound = end_idx.saturating_sub(1);
        if selected_idx < left_bound {
            let diff = (window_start_idx as isize - selected_idx as isize).abs();
            if diff <= window_size as isize {
                window_start_idx = selected_idx.saturating_sub(1);
            } else {
                let half = window_size / 2;
                window_start_idx = (selected_idx as isize - half as isize).max(0) as usize;
            }
        } else if selected_idx >= right_bound {
            let diff = (selected_idx as isize - right_bound as isize).abs();
            if diff < window_size as isize {
                window_start_idx = (selected_idx + 2).saturating_sub(window_size);
            } else {
                let half = window_size / 2;
                window_start_idx = (selected_idx as isize - half as isize).max(0) as usize;
            }
        }
        assert_eq!(window_start_idx, 0);

        // Case 2: selected_idx = 11 reaches the right edge of visible range (0..12). It should shift by 1 item to 1.
        let selected_idx: usize = 11;
        let end_idx = window_start_idx + window_size;
        let left_bound = window_start_idx + 1;
        let right_bound = end_idx.saturating_sub(1);
        if selected_idx < left_bound {
            let diff = (window_start_idx as isize - selected_idx as isize).abs();
            if diff <= window_size as isize {
                window_start_idx = selected_idx.saturating_sub(1);
            } else {
                let half = window_size / 2;
                window_start_idx = (selected_idx as isize - half as isize).max(0) as usize;
            }
        } else if selected_idx >= right_bound {
            let diff = (selected_idx as isize - right_bound as isize).abs();
            if diff < window_size as isize {
                window_start_idx = (selected_idx + 2).saturating_sub(window_size);
            } else {
                let half = window_size / 2;
                window_start_idx = (selected_idx as isize - half as isize).max(0) as usize;
            }
        }
        assert_eq!(window_start_idx, 1);

        // Case 3: selected_idx = 100 goes out of bounds by a large distance. It should center to 100 - 6 = 94.
        let selected_idx: usize = 100;
        let end_idx = window_start_idx + window_size;
        let left_bound = window_start_idx + 1;
        let right_bound = end_idx.saturating_sub(1);
        if selected_idx < left_bound {
            let diff = (window_start_idx as isize - selected_idx as isize).abs();
            if diff <= window_size as isize {
                window_start_idx = selected_idx.saturating_sub(1);
            } else {
                let half = window_size / 2;
                window_start_idx = (selected_idx as isize - half as isize).max(0) as usize;
            }
        } else if selected_idx >= right_bound {
            let diff = (selected_idx as isize - right_bound as isize).abs();
            if diff < window_size as isize {
                window_start_idx = (selected_idx + 2).saturating_sub(window_size);
            } else {
                let half = window_size / 2;
                window_start_idx = (selected_idx as isize - half as isize).max(0) as usize;
            }
        }
        assert_eq!(window_start_idx, 94);
    }
}
