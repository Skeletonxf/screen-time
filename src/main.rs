extern crate gtk;
use gtk::*;
use std::process;
use std::time::{SystemTime};

fn main() {
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let app = App::new();

    // TODO: Make this atomic so can modify it with reset button
    // and check its value from done button
    let start_time = SystemTime::now();

    {
        // program the reset button to reset the timer
        let time = app.content.time.clone();
        app.header.reset.connect_clicked(move |_| {
            time.set_label("2");
        });
    }

    {
        // program the done button to close the window
        let time = app.content.time.clone();
        app.header.done.connect_clicked(move |_| {
            time.set_label("4");
        });
    }

    // Increment the timer label every second with 500ms polls
    {
        let time = app.content.time.clone();
        gtk::timeout_add(500, move || {
            match &start_time.elapsed() {
                Ok(elapsed) => {
                    time.set_markup(&format!("<span size='200000'>{}</span>", elapsed.as_secs()));
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

        done.get_style_context().map(|c| c.add_class("suggested-action"));

        // Put the buttons on each side of the header
        container.pack_start(&reset);
        container.pack_end(&done);

        Header { container, reset, done }
    }
}
