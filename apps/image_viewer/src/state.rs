use std::path::{Path, PathBuf};
use std::time::Instant;
use zenthra_core::ObjectFit;
use crate::theme::ThemeMode;

pub struct ViewerState {
    pub images: Vec<PathBuf>,
    pub selected_idx: usize,
    pub zoom: f32,
    pub rotation: f32,
    pub grayscale_val: f32,
    pub fit_mode: ObjectFit,
    pub slideshow_active: bool,
    pub last_slide_time: Instant,
    pub sidebar_visible: bool,
    pub theme: ThemeMode,
    pub show_about: bool,
}

impl ViewerState {
    pub fn new() -> Self {
        let mut images = Vec::new();
        let dirs_to_check = [
            "/home/kabir/Pictures",
            "/home/kabir/Downloads",
            "Pictures",
            "assets/images",
        ];

        for dir_path in &dirs_to_check {
            let path = Path::new(dir_path);
            if path.exists() && path.is_dir() {
                if let Ok(entries) = std::fs::read_dir(path) {
                    let mut found_images = Vec::new();
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if entry_path.is_file() {
                            if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                                let lower = ext.to_lowercase();
                                if lower == "jpg" || lower == "jpeg" || lower == "png" || lower == "webp" {
                                    found_images.push(entry_path);
                                }
                            }
                        }
                    }
                    // Sort alphabetically for clean order
                    found_images.sort();
                    images.extend(found_images);
                }
            }
        }

        Self {
            images,
            selected_idx: 0,
            zoom: 1.0,
            rotation: 0.0,
            grayscale_val: 0.0,
            fit_mode: ObjectFit::Contain,
            slideshow_active: false,
            last_slide_time: Instant::now(),
            sidebar_visible: true,
            theme: ThemeMode::Dark,
            show_about: false,
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
    }

    pub fn load_folder(&mut self, dir_path: &Path) {
        if dir_path.exists() && dir_path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(dir_path) {
                let mut found_images = Vec::new();
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                            let lower = ext.to_lowercase();
                            if lower == "jpg" || lower == "jpeg" || lower == "png" || lower == "webp" {
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
