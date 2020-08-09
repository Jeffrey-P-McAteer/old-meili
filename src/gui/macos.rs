
use std;

use std::sync::mpsc::{channel, Sender, Receiver};
use std::{
    collections::HashMap,
    error, fmt,
};

use std::mem;
use objc::Message;

use cocoa;
use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps, NSMenu, NSMenuItem,
    NSRunningApplication, NSStatusBar, NSStatusItem, NSWindow,
};
use cocoa::base::{nil, YES /* id, class, BOOL */};

use libc;
use libc::c_void;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{sel,sel_impl,msg_send};

use objc_id;
use objc_id::Id;

use objc_foundation;
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc_foundation::{INSObject, NSObject};

use crate::config::Config;
use crate::global::Global;

pub fn open_gui(args: &Vec<String>, config: &Config, global: &Global) {
  if let Ok(mut app) = Application::new() {

    let hostname_s = format!("h: {}", config.hostname);
    app.add_menu_item(&hostname_s, |_| {
        
        // TODO real menu items
        println!("Printing a thing!");
  
        Ok::<_, Error>(())
    }).unwrap();
   
    app.add_menu_item("quit", |_| -> Result<(), Error> {
        std::process::exit(0)
    }).unwrap();


    if let Err(e) = app.wait_for_message() {
      println!("e={:?}", e);
    }
  }
}


// This allows us to send *mut Object pointers to threads
struct ObjCObjectWrapper(*mut objc::runtime::Object);
impl Default for ObjCObjectWrapper {
    fn default() -> Self {
        ObjCObjectWrapper(std::ptr::null_mut())
    }
}
unsafe impl Send for ObjCObjectWrapper {}

pub struct Window {
    name: String,
    menu: *mut objc::runtime::Object,
    pool: *mut objc::runtime::Object,
    app: *mut objc::runtime::Object,
    event_tx: Sender<SystrayEvent>
}

impl Window {
    pub fn new(event_tx: Sender<SystrayEvent>) -> Result<Window, Error> {
        let w = Window {
            name: "A".to_string(),
            menu: unsafe { NSMenu::new(nil).autorelease() },
            pool: unsafe { NSAutoreleasePool::new(nil) },
            app: unsafe { NSApp() },
            event_tx: event_tx,
        };

        unsafe {
            w.app.activateIgnoringOtherApps_(YES);
            let item = NSStatusBar::systemStatusBar(nil).statusItemWithLength_(-1.0);
            let title = NSString::alloc(nil).init_str(&w.name);
            item.setTitle_(title);
            item.setMenu_(w.menu);

            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

            // w.app.run() blocks so it needs it's own thread.
            println!("Before app.run()");
            let app = ObjCObjectWrapper(w.app);
            std::thread::spawn(move || {
                app.0.run();
            });
            println!("After app.run()");
        }

        Ok(w)
    }
    pub fn quit(&self) {
        unsafe {
            let terminate_fn = (*self.app).class().instance_method(sel!(terminate:))
                .expect("No method terminate: found")
                .implementation();
            terminate_fn();
        }
    }
    pub fn set_tooltip(&self, _: &str) -> Result<(), Error> {
        Err(Error::NotImplementedError)
    }
    pub fn add_menu_entry(&self, _item_idx: u32, item_name: &str) -> Result<(), Error> {
        unsafe {
            let cb_obj = Callback::from(Box::new(|| {
                println!("cb_obj ran from add_menu_entry");
            }));

            let no_key = NSString::alloc(nil).init_str(""); // TODO want this eventually

            let itemtitle = NSString::alloc(nil).init_str(item_name);
            let action = sel!(call);
            let item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(itemtitle, action, no_key);
            // Type inferance fails here, but we don't really
            // care about the return values so assigning
            // to _ with a () type annotation fixes a compile
            // time error
            let _: () = msg_send![item, setTarget: cb_obj];

            NSMenu::addItem_(self.menu, item);
        }
        Ok(())
    }
    pub fn add_menu_separator(&mut self, _item_idx: u32) -> Result<u32, Error> {
        Err(Error::NotImplementedError)
    }
    pub fn set_icon_from_buffer(&self, _: &[u8], _: u32, _: u32) -> Result<(), Error> {
        Err(Error::NotImplementedError)
    }
    pub fn set_icon_from_resource(&self, _: &str) -> Result<(), Error> {
        Err(Error::NotImplementedError)
    }
    pub fn set_icon_from_file(&self, file: &str) -> Result<(), Error> {
        Err(Error::NotImplementedError)
    }
    pub fn shutdown(&self) -> Result<(), Error> {
        self.quit();
        Ok(())
    }
}


// this code is pretty much a rip off of
// https://github.com/SSheldon/rust-objc-foundation/blob/master/examples/custom_class.rs

enum Callback {}
unsafe impl Message for Callback {}

// SO.. some explanation is in order here.  We want to allow closure callbacks that
// can modify their environment.  But we can't keep them on the $name object because
// that is really just a stateless proxy for the objc object.  So we store them
// as numeric pointers values in "ivar" fields on that object.  But, if we store a pointer to the
// closure object, we'll run into issues with thin/fat pointer conversions (because
// closure objects are trait objects and thus fat pointers).  So we wrap the closure in
// another boxed object ($cbs_name), which, since it doesn't use traits, is actually a
// regular "thin" pointer, and store THAT pointer in the ivar.  But...so...oy.
struct CallbackState {
    cb: Box<dyn Fn() -> ()>,
}

impl Callback {
    fn from(cb: Box<dyn Fn() -> ()>) -> Id<Self> {
        let cbs = CallbackState { cb };
        let bcbs = Box::new(cbs);

        let ptr = Box::into_raw(bcbs);
        let ptr = ptr as *mut c_void as usize;
        let mut oid = <Callback as INSObject>::new();
        (*oid).setptr(ptr);
        oid
    }

    fn setptr(&mut self, uptr: usize) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut ::objc::runtime::Object);
            obj.set_ivar("_cbptr", uptr);
        }
    }
}

// TODO: Drop for $name doesn't get called, probably because objc manages the memory and
// releases it for us.  so we leak the boxed callback right now.

impl INSObject for Callback {
    fn class() -> &'static Class {
        let cname = "Callback";

        let mut klass = Class::get(cname);
        if klass.is_none() {
            let superclass = NSObject::class();
            let mut decl = ClassDecl::new(&cname, superclass).unwrap();
            decl.add_ivar::<usize>("_cbptr");

            extern "C" fn sysbar_callback_call(this: &Object, _cmd: Sel) {
                unsafe {
                    let pval: usize = *this.get_ivar("_cbptr");
                    let ptr = pval as *mut c_void;
                    let ptr = ptr as *mut CallbackState;
                    let bcbs: Box<CallbackState> = Box::from_raw(ptr);
                    {
                        (*bcbs.cb)();
                    }
                    mem::forget(bcbs);
                }
            }

            unsafe {
                decl.add_method(
                    sel!(call),
                    sysbar_callback_call as extern "C" fn(&Object, Sel),
                );
            }

            decl.register();
            klass = Class::get(cname);
        }
        klass.unwrap()
    }
}

type BoxedError = Box<dyn error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum Error {
    OsError(String),
    NotImplementedError,
    UnknownError,
    Error(BoxedError),
}

impl From<BoxedError> for Error {
    fn from(value: BoxedError) -> Self {
        Error::Error(value)
    }
}

pub struct SystrayEvent {
    menu_index: u32,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::Error::*;

        match *self {
            OsError(ref err_str) => write!(f, "OsError: {}", err_str),
            NotImplementedError => write!(f, "Functionality is not implemented yet"),
            UnknownError => write!(f, "Unknown error occurrred"),
            Error(ref e) => write!(f, "Error: {}", e),
        }
    }
}

pub struct Application {
    window: Window,
    menu_idx: u32,
    callback: HashMap<u32, GuiCallback>,
    // Each platform-specific window module will set up its own thread for
    // dealing with the OS main loop. Use this channel for receiving events from
    // that thread.
    rx: Receiver<SystrayEvent>,
}

type GuiCallback =
    Box<(dyn FnMut(&mut Application) -> Result<(), BoxedError> + Send + Sync + 'static)>;

fn make_callback<F, E>(mut f: F) -> GuiCallback
where
    F: FnMut(&mut Application) -> Result<(), E> + Send + Sync + 'static,
    E: error::Error + Send + Sync + 'static,
{
    Box::new(move |a: &mut Application| match f(a) {
        Ok(()) => Ok(()),
        Err(e) => Err(Box::new(e) as BoxedError),
    }) as GuiCallback
}

impl Application {
    pub fn new() -> Result<Application, Error> {
        let (event_tx, event_rx) = channel();
        match Window::new(event_tx) {
            Ok(w) => Ok(Application {
                window: w,
                menu_idx: 0,
                callback: HashMap::new(),
                rx: event_rx,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn add_menu_item<F, E>(&mut self, item_name: &str, f: F) -> Result<u32, Error>
    where
        F: FnMut(&mut Application) -> Result<(), E> + Send + Sync + 'static,
        E: error::Error + Send + Sync + 'static,
    {
        let idx = self.menu_idx;
        if let Err(e) = self.window.add_menu_entry(idx, item_name) {
            return Err(e);
        }
        self.callback.insert(idx, make_callback(f));
        self.menu_idx += 1;
        Ok(idx)
    }

    pub fn add_menu_separator(&mut self) -> Result<u32, Error> {
        let idx = self.menu_idx;
        if let Err(e) = self.window.add_menu_separator(idx) {
            return Err(e);
        }
        self.menu_idx += 1;
        Ok(idx)
    }

    pub fn set_icon_from_file(&self, file: &str) -> Result<(), Error> {
        self.window.set_icon_from_file(file)
    }

    pub fn set_icon_from_resource(&self, resource: &str) -> Result<(), Error> {
        self.window.set_icon_from_resource(resource)
    }

    #[cfg(target_os = "windows")]
    pub fn set_icon_from_buffer(
        &self,
        buffer: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), Error> {
        self.window.set_icon_from_buffer(buffer, width, height)
    }

    pub fn shutdown(&self) -> Result<(), Error> {
        self.window.shutdown()
    }

    pub fn set_tooltip(&self, tooltip: &str) -> Result<(), Error> {
        self.window.set_tooltip(tooltip)
    }

    pub fn quit(&mut self) {
        self.window.quit()
    }

    pub fn wait_for_message(&mut self) -> Result<(), Error> {
        loop {
            let msg;
            match self.rx.recv() {
                Ok(m) => msg = m,
                Err(_) => {
                    self.quit();
                    break;
                }
            }
            if self.callback.contains_key(&msg.menu_index) {
                if let Some(mut f) = self.callback.remove(&msg.menu_index) {
                    f(self)?;
                    self.callback.insert(msg.menu_index, f);
                }
            }
        }

        Ok(())
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        self.shutdown().ok();
    }
}

