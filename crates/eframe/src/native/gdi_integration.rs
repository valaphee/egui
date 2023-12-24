use std::sync::Arc;

use winit::event_loop::{EventLoop, EventLoopProxy};

use crate::{AppCreator, NativeOptions, UserEvent};

use super::{epi_integration::EpiIntegration, winit_integration::WinitApp};

pub struct GdiWinitApp {
    repaint_proxy: Arc<egui::mutex::Mutex<EventLoopProxy<UserEvent>>>,
    app_name: String,
    native_options: NativeOptions,
    app_creator: Option<AppCreator>,
    running: Option<GdiWinitRunning>,
}

struct GdiWinitRunning {
    integration: EpiIntegration,
}

impl GdiWinitApp {
    pub fn new(
        event_loop: &EventLoop<UserEvent>,
        app_name: &str,
        native_options: NativeOptions,
        app_creator: AppCreator,
    ) -> Self {
        crate::profile_function!();

        #[cfg(feature = "__screenshot")]
        assert!(
            std::env::var("EFRAME_SCREENSHOT_TO").is_err(),
            "EFRAME_SCREENSHOT_TO not yet implemented for gdi backend"
        );

        Self {
            repaint_proxy: Arc::new(egui::mutex::Mutex::new(event_loop.create_proxy())),
            app_name: app_name.to_owned(),
            native_options,
            app_creator: Some(app_creator),
            running: None,
        }
    }
}

impl WinitApp for GdiWinitApp {
    fn frame_nr(&self, viewport_id: egui::ViewportId) -> u64 {
        todo!()
    }

    fn is_focused(&self, window_id: winit::window::WindowId) -> bool {
        todo!()
    }

    fn integration(&self) -> Option<&EpiIntegration> {
        todo!()
    }

    fn window(
        &self,
        window_id: winit::window::WindowId,
    ) -> Option<std::rc::Rc<winit::window::Window>> {
        todo!()
    }

    fn window_id_from_viewport_id(&self, id: egui::ViewportId) -> Option<winit::window::WindowId> {
        todo!()
    }

    fn save_and_destroy(&mut self) {
        todo!()
    }

    fn run_ui_and_paint(
        &mut self,
        event_loop: &winit::event_loop::EventLoopWindowTarget<UserEvent>,
        window_id: winit::window::WindowId,
    ) -> super::winit_integration::EventResult {
        todo!()
    }

    fn on_event(
        &mut self,
        event_loop: &winit::event_loop::EventLoopWindowTarget<UserEvent>,
        event: &winit::event::Event<UserEvent>,
    ) -> crate::Result<super::winit_integration::EventResult> {
        todo!()
    }
}
