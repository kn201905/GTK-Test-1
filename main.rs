mod custom_button;

use gtk::prelude::*;
use gtk::{ glib, Application, ApplicationWindow, Button };
use std::{ cell::Cell, rc::Rc };

use custom_button::CustomButton;

const APP_ID: &str = "gtk_rs.Sample";

fn main() -> glib::ExitCode {
	let app = Application::builder().application_id(APP_ID).build();

	// Connect to "activate" signal of `app`
	app.connect_activate(build_ui);
	app.run()
}


fn build_ui(app: &Application) {
	let button_increase = Button::builder()
		.label("Increase")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	let button_decrease = gtk::Button::builder()
		.label("Decrease")
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();
		  
	let number = Rc::new(Cell::new(0));
	
	// --------------------------------------------
   button_increase.connect_clicked(
		glib::clone!(
			#[strong] number,
			#[weak] button_decrease,
			move |_| {
				number.set(number.get() + 1);
				button_decrease.set_label(&number.get().to_string());
			}
		));
		
	button_decrease.connect_clicked(
		glib::clone!(
			#[strong] number,
			#[weak] button_increase,
			move |_| {
				number.set(number.get() - 1);
				button_increase.set_label(&number.get().to_string());
			}
		));

	// --------------------------------------------
	let btn_custom = CustomButton::with_label("Click me!");
	btn_custom.connect_clicked(|btn| {
		let num: u32 = btn.clicked_count();
		println!("clicked_count -> {}", num);
		btn.set_clicked_count(num + 1);
	});

	assert_eq!(btn_custom.property::<String>("label"), "Click me!");
	btn_custom.set_property("label", "Changed");

	// --------------------------------------------
	let gtk_box = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.build();

	gtk_box.append(&button_increase);
	gtk_box.append(&button_decrease);
	gtk_box.append(&btn_custom);
		  
	let window = ApplicationWindow::builder()
		.application(app)
		.title("My GTK App")
		.child(&gtk_box)
		.build();

	window.present();
}

