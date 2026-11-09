use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]

struct App { //struct App defines a type that will hold the application’s state between event callbacks.
    window: Option<Window>, //The App struct then owns the window and keeps it alive between callbacks.
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title("Anakena");
        let window = event_loop.create_window(attributes).expect("Failed to create window");
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) { //Defines the callback Winit calls when a window receives an event.
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

fn main () {
    let event_loop = EventLoop::new().expect("Failed to create event loop"); //We’ll use event_loop to dispatch window events once we have a handler.
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Event loop failed"); //run_app starts dispatching events to App
}