
use std;

use std::sync::mpsc::{channel, Sender};

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
  let menu: *mut objc::runtime::Object = unsafe { NSMenu::new(nil).autorelease() };
  let pool: *mut objc::runtime::Object = unsafe { NSAutoreleasePool::new(nil) };



}

