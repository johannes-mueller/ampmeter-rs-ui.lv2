
extern crate lv2_sys;
extern crate lv2_atom;
extern crate lv2_urid;
extern crate lv2_core;
extern crate urid;

extern crate lv2_ui;

extern crate pugl_ui;
extern crate cairo;
extern crate pango;

mod dial;
mod button;
mod meter;

use lv2_ui::*;
use lv2_sys::*;
use lv2_atom::prelude::*;
use urid::*;
use lv2_urid::*;
use lv2_core::prelude::*;

//use pugl_ui::widget::*;
use pugl_ui::ui::*;
use pugl_ui::layout::*;
use pugl_ui::*;
use pugl_sys::*;

#[derive(FeatureCollection)]
struct Features<'a> {
    map: LV2Map<'a>
}

#[derive(URIDCollection)]
struct URIDs {
    atom: AtomURIDCollection,
}

struct MyUIPorts {
    gain: UIPort<f32>,
    enabled: UIPort<f32>,
    meter_in: UIPort<f32>,
    meter_out: UIPort<f32>
}

impl UIPortsTrait for MyUIPorts {
    fn port_map(&mut self, port_index: u32) -> Option<UIPortRaw> {
	match port_index {
	    0 => Some(self.gain.value_as_ptr()),
	    1 => Some(self.enabled.value_as_ptr()),
	    2 => Some(self.meter_in.value_as_ptr()),
	    3 => Some(self.meter_out.value_as_ptr()),
	    _ => None
	}
    }
}

#[uri("https://johannes-mueller.org/lv2/ampmeter-rs#ui")]
struct AmpUI {
    view: Box<PuglView<UI<RootWidget>>>,

    gain_dial: widget::WidgetHandle<dial::Dial>,
    enable_btn: widget::WidgetHandle<button::Button>,
    meter_in: widget::WidgetHandle<meter::Meter>,
    meter_out: widget::WidgetHandle<meter::Meter>,
    ports: MyUIPorts,

    urids: URIDs
}

impl AmpUI {
    pub fn new(features: &mut Features<'static>, parent_window: *mut std::ffi::c_void) -> Option<Self> {
	eprintln!("new");
	let mut ui = Box::new(UI::new(Box::new(RootWidget::default())));

	let h_layout = ui.new_layouter::<HorizontalLayouter>();
	let v_layout = ui.new_layouter::<VerticalLayouter>();

	let gain_dial = ui.new_widget(dial::Dial::new(-90., 24., 1.));
	let enable_btn = ui.new_widget(button::Button::new_toggle_button("enable", false));
	let meter_in = ui.new_widget(meter::Meter::new(-60., 20.));
	let meter_out = ui.new_widget(meter::Meter::new(-60., 20.));

	ui.pack_to_layout(h_layout.widget(), ui.root_layout(), StackDirection::Front);
	ui.pack_to_layout(v_layout.widget(), h_layout, StackDirection::Front);
	ui.pack_to_layout(gain_dial, v_layout, StackDirection::Front);
	ui.pack_to_layout(enable_btn, v_layout, StackDirection::Front);
	ui.pack_to_layout(meter_in, h_layout, StackDirection::Back);
	ui.pack_to_layout(meter_out, h_layout, StackDirection::Back);
	ui.do_layout();

	let view = PuglView::make_view(ui, parent_window);

	let ui = view.handle();
	ui.fit_window_size();
	ui.fit_window_min_size();
	ui.set_window_title("ampmeter");
	ui.show_window();

	let ports = MyUIPorts {
	    gain: UIPort::<f32>::new(),
	    enabled: UIPort::<f32>::new(),
	    meter_in: UIPort::<f32>::new(),
	    meter_out: UIPort::<f32>::new()
	};

	Some(Self {
	    view,
	    gain_dial,
	    enable_btn,
	    meter_in,
	    meter_out,
	    ports,
	    urids: features.map.populate_collection()?
	})
    }

    fn ui(&self) -> &mut UI<RootWidget> {
	self.view.handle()
    }
}

impl PluginUI for AmpUI {

    type InitFeatures = Features<'static>;
    type UIPorts = MyUIPorts;

    fn new(plugin_ui_info: &PluginUIInfo, features: &mut Self::InitFeatures, parent_window: *mut std::ffi::c_void) -> Option<Self> {
	eprintln!("AmpUI::new()");
	Self::new(features, parent_window)
    }

    fn cleanup(&mut self) {
	eprintln!("cleanup called");
    }

    fn ports(&mut self) -> &mut MyUIPorts {
	&mut self.ports
    }

    fn widget(&self) -> LV2UI_Widget {
	eprintln!("AmpUI::widget() {:?}", self.view.native_window() as *const std::ffi::c_void);

	self.view.native_window() as LV2UI_Widget
    }

    fn idle(&mut self) -> i32 {
	let ui = self.ui();
	ui.next_event(0.0);

	if ui.close_request_issued() {
	    return 1;
	}

	if ui.root_widget().focus_next() {
		ui.focus_next_widget();
	}

	if let Some(v) = self.ui().widget(self.gain_dial).changed_value() {
	    self.ports.gain.set_value(v as f32);
	}

	if let Some(v) = self.ui().widget(self.enable_btn).toggle_state() {
	    self.ports.enabled.set_value( if v { 1.0 } else { 0.0 } );
	}

	0
    }

    fn update(&mut self) {
	if let Some(v) = self.ports.gain.value() {
	    self.ui().widget(self.gain_dial).set_value((v as f32).into());
	}
	if let Some(v) = self.ports.enabled.value() {
	    self.ui().widget(self.enable_btn).set_toggle_state(v > 0.5);
	}
	if let Some(v) = self.ports.meter_in.value() {
	    self.ui().widget(self.meter_in).set_value(v);
	}
	if let Some(v) = self.ports.meter_out.value() {
	    self.ui().widget(self.meter_out).set_value(v);
	}
    }
}


unsafe impl PluginUIInstanceDescriptor for AmpUI {
    const DESCRIPTOR: LV2UI_Descriptor = LV2UI_Descriptor {
	URI: Self::URI.as_ptr() as *const u8 as *const ::std::os::raw::c_char,
	instantiate: Some(PluginUIInstance::<Self>::instantiate),
	cleanup: Some(PluginUIInstance::<Self>::cleanup),
	port_event: Some(PluginUIInstance::<Self>::port_event),
	extension_data: Some(PluginUIInstance::<Self>::extension_data)
    };
}

#[no_mangle]
pub unsafe extern "C" fn lv2ui_descriptor(index: u32) -> *const LV2UI_Descriptor {
    eprintln!("my ui descriptor called {}", index);
    match index {
	0 => &<AmpUI as PluginUIInstanceDescriptor>::DESCRIPTOR,
	_ => std::ptr::null()
    }
}

#[derive(Default)]
struct RootWidget {
    stub: widget::WidgetStub,
    focus_next: bool
}

impl widget::Widget for RootWidget {
    widget_stub!();
    fn exposed (&self, _expose: &ExposeArea, cr: &cairo::Context) {
        cr.set_source_rgb (0.2, 0.2, 0.2);
        let size = self.size();
        cr.rectangle (0., 0., size.w, size.h);
        cr.fill ();
    }
    fn event(&mut self, ev: Event) -> Option<Event> {
        ev.try_keypress()
            .and_then(|kp| kp.try_char())
            .and_then(|c| {
                match c {
                    '\t' => {
                        self.focus_next = true;
                        event_processed!()
                    },
                    _ => event_not_processed!()
                }
            })
            .or(event_not_processed!()).and_then (|p| p.pass_event (ev))
    }
}

impl RootWidget {
    pub fn focus_next(&mut self) -> bool {
	let f = self.focus_next;
	self.focus_next = false;
	f
    }
}
