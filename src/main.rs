use winit::application::ApplicationHandler;
use winit::event::WindowEvent; //means “the WindowEvent type in the event module of winit.”
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

mod renderer;

#[derive(Default)]

struct App { //struct App defines a type that will hold the application’s state between event callbacks.
    window: Option<std::sync::Arc<Window>>, //The App struct then owns the window and keeps it alive between callbacks.
    renderer: Option<renderer::Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_title("Anakena");
        let window = event_loop.create_window(attributes).expect("Failed to create window"); //opens the OS window and gives you a Window.
        let window = std::sync::Arc::new(window); //moves that same Window into an Arc<Window> so the app and renderer can share ownership. The new window binding takes over the name. Rust calls this shadowing.
        self.renderer = Some(pollster::block_on(renderer::Renderer::new(window.clone())));
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) { //Defines the callback Winit calls when a window receives an event.
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
        else if let WindowEvent::RedrawRequested = event && let Some(renderer) = &self.renderer {
            renderer.render()
        }
    }
}

fn main () {
    let event_loop = EventLoop::new().expect("Failed to create event loop"); //We’ll use event_loop to dispatch window events once we have a handler.
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Event loop failed"); //run_app starts dispatching events to App
}