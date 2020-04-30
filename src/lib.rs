
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

use lv2_ui::*;
use lv2_sys::*;
use lv2_atom::prelude::*;
use urid::*;
use lv2_urid::*;
use lv2_core::prelude::*;

use pugl_ui::widget::*;
use pugl_ui::ui::*;
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
}

impl UIPortsTrait for MyUIPorts {
    fn port_map(&mut self, port_index: u32) -> Option<UIPortRaw> {
	match port_index {
	    0 => Some(self.gain.value_as_ptr()),
	    1 => Some(self.enabled.value_as_ptr()),
	    _ => None
	}
    }
}

#[uri("https://johannes-mueller.org/lv2/ampmeter-rs#ui")]
struct AmpUI {
    view: Box<PuglView<UI>>,

    gain_dial: Id,
    enable_btn: Id,
    ports: MyUIPorts,

    urids: URIDs
}

impl AmpUI {
    pub fn new(features: &mut Features<'static>, parent_window: *mut std::ffi::c_void) -> Option<Self> {
	eprintln!("new");
	let mut ui = Box::new(UI::new( RootWidgetFactory {}, Layouter::Vertical(StackLayouter::default())));

	let gain_dial = ui.new_widget(0, dial::new(-90., 24., 1.));
	let enable_btn = ui.new_widget(0, button::new_toggle_button("enable", false));

	ui.pack_to_layout(gain_dial, LayoutTarget::Vertical(LayoutDirection::Front));
	ui.pack_to_layout(enable_btn, LayoutTarget::Vertical(LayoutDirection::Front));
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
	};

	Some(Self { view, gain_dial, enable_btn, ports, urids: features.map.populate_collection()? })
    }

    fn ui(&self) -> &mut UI {
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
	ui.next_event(-1.0);

	if ui.widget::<RootWidget>(0).focus_next() {
		ui.focus_next_widget();
	}

	if let Some(v) = self.ui().widget::<dial::Dial>(self.gain_dial).changed_value() {
	    self.ports.gain.set_value(v as f32);
	}

	if let Some(v) = self.ui().widget::<button::Button>(self.enable_btn).toggle_state() {
	    self.ports.enabled.set_value( if v { 1.0 } else { 0.0 } );
	}

	0
    }

    fn update(&mut self) {
	if let Some(v) = self.ports.gain.value() {
	    self.ui().widget::<dial::Dial>(self.gain_dial).set_value((v as f32).into());
	}
	if let Some(v) = self.ports.enabled.value() {
	    self.ui().widget::<button::Button>(self.enable_btn).set_toggle_state(v > 0.5);
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


struct RootWidget {
    stub: WidgetStub,
    focus_next: bool
}

impl Widget for RootWidget {
    fn exposed (&self, _expose: &ExposeArea, cr: &cairo::Context) {
        cr.set_source_rgb (0.2, 0.2, 0.2);
        let size = self.size();
        cr.rectangle (0., 0., size.w, size.h);
        cr.fill ();
    }
    fn min_size(&self) -> Size { Size { w: 0., h: 0. } }
    fn stub (&self) -> &WidgetStub {
        &self.stub
    }
    fn stub_mut (&mut self) -> &mut WidgetStub {
        &mut self.stub
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

struct RootWidgetFactory {}
impl WidgetFactory<RootWidget> for RootWidgetFactory {
    fn make_widget(&self, stub: WidgetStub) -> RootWidget {
        RootWidget {
            stub,
	    focus_next: false
        }
    }
}
