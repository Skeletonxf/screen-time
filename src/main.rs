extern crate gtk;
use gtk::*;
use std::process;
use std::time::{SystemTime};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
// UI <-> App state operations use Sequentially Consistent ordering
// https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst
use std::sync::atomic::Ordering;

fn main() {
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Use an atomic reference counter so we can use this from each closure
    let state = Arc::new(Timer::new());

    let app = App::new();

    let start_time = SystemTime::now();

    {
        // program the reset button to reset the timer
        let time = app.content.time.clone();
        let state = state.clone();
        let done = app.header.done.clone();

        app.header.reset.connect_clicked(move |_| {
            match &start_time.elapsed() {
                Ok(elapsed) => {
                    let seconds = elapsed.as_secs() - state.get();

                    // update the UI back to 0 seconds
                    time.set_markup("<span size='200000'>0</span>");
                    // update the state to ignore all the elapsed seconds so far
                    state.set(elapsed.as_secs());

                    // make sure the done button is no longer green when resetting
                    if seconds >= 20 {
                        done.get_style_context().map(|c| c.remove_class("suggested-action"));
                    }
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        });
    }

    {
        // program the done button to close the window if the time is >= 20 seconds
        let state = state.clone();

        app.header.done.connect_clicked(move |_| {
            match &start_time.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_secs() - state.get() >= 20 {
                        main_quit();
                        Inhibit(false);
                    }
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        });
    }

    // Increment the timer label every second with 500ms polls
    {
        let time = app.content.time.clone();
        let state = state.clone();
        let done = app.header.done.clone();

        gtk::timeout_add(500, move || {
            match &start_time.elapsed() {
                Ok(elapsed) => {
                    // instead of trying to make the system time atomic
                    // we store the seconds since starting the program to
                    // ignore atomically instead, which provides the same
                    // function
                    let seconds = elapsed.as_secs() - state.get();

                    time.set_markup(&format!("<span size='200000'>{}</span>", seconds));

                    // make the done button go green when ready
                    if seconds >= 20 {
                        done.get_style_context().map(|c| c.add_class("suggested-action"));
                    }
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            Continue(true)
        });
    }

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
}

pub struct Timer {
    pub time: AtomicU64,
}

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
    pub reset: Button,
    pub done: Button,
}

pub struct Content {
    pub container: Box,
    pub time: Label,
}

impl Timer {
    fn new() -> Timer {
        Timer { time: AtomicU64::new(0) }
    }

    fn get(&self) -> u64 {
        self.time.load(Ordering::SeqCst)
    }

    fn set(&self, value: u64) {
        self.time.store(value, Ordering::SeqCst);
    }
}

impl Content {
    fn new() -> Content {
        // Store the content vertially
        let container = Box::new(Orientation::Vertical, 0);

        let time_info = Box::new(Orientation::Horizontal, 0);
        let time_label = Label::new("s");
        let time = Label::new("0");

        // Center the time info with the label after the value
        time_info.set_halign(Align::Center);
        time_label.set_halign(Align::Start);
        time.set_halign(Align::Start);

        // Adjust the font sizing using HTML markup
        time.set_markup("<span size='200000'>0</span>");
        time_label.set_markup("<span size='xx-large'>s</span>");

        // Put the time and label into the info box
        time_info.pack_start(&time, false, false, 5);
        // `s` should be after the changing time
        time_info.pack_start(&time_label, true, true, 5);

        // Add everything to the vertical box
        container.pack_start(&time_info, true, false, 0);

        // Put the time as a field in the Content so we can manipulate it
        // from the UI
        Content { container, time }
    }
}

impl App {
    fn new() -> App {
        // Create a new top level window.
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        // Set the headerbar as the title bar widget.
        window.set_titlebar(&header.container);
        // Set the title of the window.
        window.set_title("Screen Time");
        // Set the window manager class.
        window.set_wmclass("screen-time", "Screen Time");
        // The icon the app will display.
        Window::set_default_icon_name("iconname");
        // Set the default size of the window.
        window.set_default_size(800, 500);
        // Add the content box into the window.
        window.add(&content.container);

        // Close the app when exit button is used
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, header, content }
    }
}

impl Header {
    fn new() -> Header {
        // Create the main header bar container widget.
        let container = HeaderBar::new();

        container.set_title("Screen Time");

        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        // Create the two buttons for the UI
        let reset = Button::new_with_label("Reset");
        let done = Button::new_with_label("Done");

        // Put the buttons on each side of the header
        container.pack_start(&reset);
        container.pack_end(&done);

        Header { container, reset, done }
    }
}
