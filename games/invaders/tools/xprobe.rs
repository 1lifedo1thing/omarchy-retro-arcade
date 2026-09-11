// Native X11 verification helper. Compile with rustc; no application dependency.
use std::{ffi::{c_char,c_int,c_uint,c_ulong,c_void,CString,CStr},thread,time::Duration};
type Display=c_void;
#[link(name="libX11.so.6",kind="dylib",modifiers="+verbatim")]
unsafe extern "C" {
 fn XOpenDisplay(name:*const c_char)->*mut Display;
 fn XDefaultRootWindow(d:*mut Display)->c_ulong;
 fn XQueryTree(d:*mut Display,w:c_ulong,root:*mut c_ulong,parent:*mut c_ulong,children:*mut *mut c_ulong,count:*mut c_uint)->c_int;
 fn XFetchName(d:*mut Display,w:c_ulong,name:*mut *mut c_char)->c_int;
 fn XFree(p:*mut c_void)->c_int;
 fn XSetInputFocus(d:*mut Display,w:c_ulong,revert:c_int,time:c_ulong)->c_int;
 fn XStringToKeysym(s:*const c_char)->c_ulong;
 fn XKeysymToKeycode(d:*mut Display,k:c_ulong)->u8;
 fn XFlush(d:*mut Display)->c_int;
 fn XResizeWindow(d:*mut Display,w:c_ulong,width:c_uint,height:c_uint)->c_int;
}
#[link(name="libXtst.so.6",kind="dylib",modifiers="+verbatim")]
unsafe extern "C" {fn XTestFakeMotionEvent(d:*mut Display,screen:c_int,x:c_int,y:c_int,delay:c_ulong)->c_int; fn XTestFakeButtonEvent(d:*mut Display,button:c_uint,press:c_int,delay:c_ulong)->c_int; fn XTestFakeKeyEvent(d:*mut Display,k:c_uint,press:c_int,delay:c_ulong)->c_int;}
fn main(){unsafe{
 let d=XOpenDisplay(std::ptr::null());assert!(!d.is_null());let root=XDefaultRootWindow(d);let mut r=0;let mut parent=0;let mut kids=std::ptr::null_mut();let mut n=0;XQueryTree(d,root,&mut r,&mut parent,&mut kids,&mut n);
 let mut win=0;for i in 0..n {let w=*kids.add(i as usize);let mut name=std::ptr::null_mut();XFetchName(d,w,&mut name);if !name.is_null(){if CStr::from_ptr(name).to_bytes()==b"Omarchy Invaders"{win=w;}XFree(name.cast());}}if !kids.is_null(){XFree(kids.cast());}assert_ne!(win,0,"Window not found");
 let args:Vec<_>=std::env::args().collect();let cmd=args.get(1).map(String::as_str).unwrap_or("id");
 match cmd {"id"=>println!("{win}"),"blur"=>{XSetInputFocus(d,root,1,0);},"resize"=>{XResizeWindow(d,win,args[2].parse().unwrap(),args[3].parse().unwrap());},_=>{XSetInputFocus(d,win,1,0);XFlush(d);thread::sleep(Duration::from_millis(100));if cmd=="click" { XTestFakeMotionEvent(d,-1,args[2].parse().unwrap(),args[3].parse().unwrap(),0); XTestFakeButtonEvent(d,1,1,0); XFlush(d); thread::sleep(Duration::from_millis(60)); XTestFakeButtonEvent(d,1,0,0); } if cmd=="key"{let ctrl=args.iter().any(|a|a=="ctrl");let k=XKeysymToKeycode(d,XStringToKeysym(CString::new(args[2].as_str()).unwrap().as_ptr()))as u32;let control=XKeysymToKeycode(d,0xffe3)as u32;if ctrl{XTestFakeKeyEvent(d,control,1,0);}XTestFakeKeyEvent(d,k,1,0);XFlush(d);thread::sleep(Duration::from_millis(args.get(3).and_then(|s|s.parse().ok()).unwrap_or(60)));XTestFakeKeyEvent(d,k,0,0);if ctrl{XTestFakeKeyEvent(d,control,0,0);}}}}
 XFlush(d);thread::sleep(Duration::from_millis(100));
}}
