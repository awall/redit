#![feature(globs)]

extern crate libc;

use libc::types::os::arch::c95::{ c_int, c_uint, c_long, c_ulong, c_ushort, c_char, wchar_t }; // , c_short, c_uchar, c_schar, c_float, c_double
//use libc::types::os::arch::c99::{ c_longlong, c_ulonglong };
use libc::types::common::c95::c_void;
use std::char;
use std::mem::transmute;

use geometry::*;
use buffer::*;

mod geometry;
mod buffer;

type BOOL = c_int;
type BYTE = u8;
type CHAR = c_char;
type DWORD = c_ulong;
type LONG = c_long;
type UINT = c_uint;
type WCHAR = wchar_t;
type WORD = c_ushort;

#[cfg(target_arch = "x86")]    #[allow(non_camel_case_types)] type LONG_PTR = c_long;
#[cfg(target_arch = "x86_64")] #[allow(non_camel_case_types)] type LONG_PTR = i64;
#[cfg(target_arch = "x86")]    #[allow(non_camel_case_types)] type UINT_PTR = c_uint;
#[cfg(target_arch = "x86_64")] #[allow(non_camel_case_types)] type UINT_PTR = u64;

type PVOID = *mut c_void;
type LPCWSTR = *const WCHAR;
type LPSTR = *mut CHAR;
type LPVOID = *mut c_void;

type ATOM = WORD;
type LPARAM = LONG_PTR;
type LRESULT = LONG_PTR;
type WPARAM = UINT_PTR;
type WNDPROC = *const c_void;

type HANDLE = PVOID;
type HBRUSH = HANDLE;
type HCURSOR = HANDLE;
type HDC = HANDLE;
type HGDIOBJ = HANDLE;
type HICON = HANDLE;
type HINSTANCE = HANDLE;
type HMENU = HANDLE;
type HMODULE = HANDLE;
type HWND = HANDLE;

type COLORREF = DWORD;

// static WS_BORDER: u32 = 0x800000;
// static WS_CAPTION: u32 = 0xc00000;
// static WS_CHILD: u32 = 0x40000000;
// static WS_CHILDWINDOW: u32 = 0x40000000;
// static WS_CLIPCHILDREN: u32 = 0x2000000;
// static WS_CLIPSIBLINGS: u32 = 0x4000000;
// static WS_DISABLED: u32 = 0x8000000;
// static WS_DLGFRAME: u32 = 0x400000;
// static WS_GROUP: u32 = 0x20000;
// static WS_HSCROLL: u32 = 0x100000;
// static WS_ICONIC: u32 = 0x20000000;
// static WS_MAXIMIZE: u32 = 0x1000000;
// static WS_MAXIMIZEBOX: u32 = 0x10000;
// static WS_MINIMIZE: u32 = 0x20000000;
// static WS_MINIMIZEBOX: u32 = 0x20000;
// static WS_OVERLAPPED: u32 = 0;
static WS_OVERLAPPEDWINDOW: u32 = 0xcf0000;
// static WS_POPUP: u32 = 0x80000000;
// static WS_POPUPWINDOW: u32 = 0x80880000;
// static WS_SIZEBOX: u32 = 0x40000;
// static WS_SYSMENU: u32 = 0x80000;
// static WS_TABSTOP: u32 = 0x10000;
// static WS_THICKFRAME: u32 = 0x40000;
// static WS_TILED: u32 = 0;
// static WS_TILEDWINDOW: u32 = 0xcf0000;
// static WS_VISIBLE: u32 = 0x10000000;
// static WS_VSCROLL: u32 = 0x200000;

const WM_CHAR: UINT = 0x0102;
const WM_DESTROY: UINT = 0x0002;
const WM_PAINT: UINT = 0x000F;

const VK_RETURN: UINT = 0x000D;
const VK_BACK:   UINT = 0x0008;
const VK_LEFT:   UINT = 0x0025;
const VK_UP:     UINT = 0x0026;
const VK_RIGHT:  UINT = 0x0027;
const VK_DOWN:   UINT = 0x0028;

const TRANSPARENT: c_int = 1;
const OPAQUE:      c_int = 2;

const WHITE_BRUSH: c_int = 0;
const DC_BRUSH: c_int = 18;
const DC_PEN:   c_int = 19;
// 0      WHITE_BRUSH
// 1      GRAY_BRUSH
// 2      LTGRAY_BRUSH
// 3      DKGRAY_BRUSH
// 4      BLACK_BRUSH
// 5      NULL_BRUSH (also HOLLOW_BRUSH)
// 6      WHITE_PEN
// 7      BLACK_PEN
// 8      NULL_PEN
// 10     OEM_FIXED_FONT
// 11     ANSI_FIXED_FONT
// 12     ANSI_VAR_FONT
// 13     SYSTEM_FONT
// 14     DEVICE_DEFAULT_FONT
// 15     DEFAULT_PALETTE
// 16     SYSTEM_FIXED_FONT
// 17     DEFAULT_GUI_FONT
// 18	  DC_BRUSH
// 19     DC_PEN

#[allow (non_snake_case)]
#[repr(C)]
struct WNDCLASSEX {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
    hIconSm: HICON,
}

#[allow (non_snake_case)]
#[repr(C)]
struct MSG {
   hwnd: HWND,
   message: UINT,
   wParam: WPARAM,
   lParam: LPARAM,
   time: DWORD,
   pt: POINT,
}

#[allow (non_snake_case)]
#[repr(C)]
struct PAINTSTRUCT {
   hdc: HDC,
   fErase: BOOL,
   rcPaint: RECT,
   fRestore: BOOL,
   fIncUpdate: BOOL,
   rgbReserved: [BYTE, ..32],
}

#[repr(C)]
struct POINT {
	x: LONG,
	y: LONG,
}

#[repr(C)]
struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

#[link (name="kernel32")]
extern "system" {
	fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

#[link (name="user32")]
extern "system" {
    fn CreateWindowExW(
        extrastyle: DWORD, 
        classname: LPCWSTR, 
        windowname: LPCWSTR, 
        style: DWORD, 
        x: c_int, 
        y: c_int, 
        width: c_int, 
        height: c_int, 
        parent: HWND, 
        menu: HMENU, 
        instance: HINSTANCE, 
        param: LPVOID,
    ) -> HWND;

	fn GetMessageW(
		lpMsg: *const MSG,
		hWnd: HWND,
		wMsgFilterMin: UINT,
		wMsgFilterMAx: UINT,
	) -> BOOL;

	fn DefWindowProcW(
         hWnd: HWND,
         Msg: UINT,
         wParam: WPARAM,
         lParam: LPARAM,
	) -> LRESULT;

	fn RegisterClassExW(lpwcx: *const WNDCLASSEX) -> ATOM;
    fn ShowWindow(hwnd: HWND, nCmdShow: c_int) -> BOOL;
    fn UpdateWindow(hwnd: HWND) -> BOOL;
    fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
	fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
	fn PostQuitMessage(nExitCode: c_int);
	fn BeginPaint(hwnd: HWND, lpPaint: *mut PAINTSTRUCT) -> HDC;
	fn EndPaint(hwnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
	fn InvalidateRect(hwnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
	fn GetStockObject(fnObject: c_int) -> HGDIOBJ;
}

#[link (name="gdi32")]
extern "system" {
	fn TextOutW(
  		hdc: HDC,
  		nXStart: c_int,
  		nYStart: c_int,
  	    lpString: LPCWSTR,
  	    cchString: c_int,
	) -> BOOL;

	fn Rectangle(
		hdc: HDC,
		left: c_int,
		top: c_int,
		right: c_int,
		bottom: c_int,
	) -> BOOL;

	fn SetBkMode(
		hdc: HDC,
		iBkMode: c_int,
	) -> c_int;

	fn SetDCBrushColor(
  		hdc: HDC,
  		crColor: COLORREF,
	) -> COLORREF;

	fn SetDCPenColor(
  		hdc: HDC,
  		crColor: COLORREF,
	) -> COLORREF;

	fn SelectObject(
		hdc: HDC,
		hgdiObj: HGDIOBJ,
	) -> HGDIOBJ;
}


static mut g_buffer: *mut Buffer = 0 as *mut Buffer;

#[allow(unused_mut)]
fn main() { unsafe {
    let instance = GetModuleHandleW(0 as LPCWSTR);

	let icon = 0 as HICON;
	let icon_small = 0 as HICON;
	let cursor = 0 as HCURSOR;
    let background_brush = GetStockObject(WHITE_BRUSH) as HBRUSH;
    let menu_name = 0 as LPCWSTR;
    let class_name: Vec<u16> = "Window Class".utf16_units().chain(Some(0).into_iter()).collect::<Vec<u16>>();

    let wndclass = WNDCLASSEX {
    	cbSize: std::mem::size_of::<WNDCLASSEX>() as UINT,
    	style: 3 as UINT, // CS_HREDRAW | CS_VREDRAW;
    	lpfnWndProc: window_proc as WNDPROC,
    	cbClsExtra: 0,
    	cbWndExtra: 0,
    	hInstance: instance,
    	hIcon: icon,
    	hCursor: cursor,
    	hbrBackground: background_brush,
    	lpszMenuName: menu_name,
    	lpszClassName: class_name.as_ptr(),
    	hIconSm: icon_small,
    };

    RegisterClassExW(&wndclass);

    let caption = 0 as LPCWSTR;

    let hwnd = CreateWindowExW(
        0 as DWORD, // extrastyle
        class_name.as_ptr(),
        caption,
		WS_OVERLAPPEDWINDOW, //style
		50, 50, // x, y
		200, 200, // width, height
		0 as HWND, 0 as HMENU, // parent, menu
		instance, 
		0 as LPVOID); // param 	
	
    ShowWindow(hwnd, 5); // WS_SHOW
    UpdateWindow(hwnd);

    let msg = MSG {
	    hwnd: 0 as HWND,
			message: 0 as UINT,
			wParam: 0 as WPARAM,
			lParam: 0 as LPARAM,
			time: 0 as DWORD,
			pt: POINT {
				x: 0, 
				y:0,
			},
    };

	let mut buffer = Buffer {
		show_line_numbers: true,
		cursor: Pos { x: 0, y: 0 },
		lines: vec![Line { text: String::new() }],
	};

	g_buffer = transmute(&buffer);

    while GetMessageW(&msg, 0 as HWND, 0, 0) > 0 {
		TranslateMessage(&msg);
		DispatchMessageW(&msg);			
    }

    std::os::set_exit_status(0); 
}}

struct TextBlock {
	text: String,
}

struct Canvas {
	rects: Vec<Rect>,
	text_blocks: Vec<(Pos, TextBlock)>,
}

fn stage(buffer: &Buffer) -> Canvas {
	let mut text_blocks: Vec<(Pos, TextBlock)> = vec![];

	let mut i = 0;
	for line in buffer.lines.iter() {
		let text = line.text.as_slice().to_string();
		text_blocks.push((Pos { x: 0, y: i }, TextBlock { text: text }));
		i += 14;
	}

	let curx = buffer.cursor.x;
	let cury = buffer.cursor.y;
	let mut rects: Vec<Rect> = vec![
		Rect { 
			lt: Pos { x:  curx    * 10, y:  cury    * 14 }, 
			rb: Pos { x: (curx+1) * 10, y: (cury+1) * 14 },
		},
	];

	Canvas {
		rects: rects,
		text_blocks: text_blocks,
	}
}

unsafe fn paint(hwnd: HWND, canvas: &Canvas) {
	let mut ps = PAINTSTRUCT {
		hdc: 0 as HDC,
		fErase: 0 as BOOL,
		rcPaint: RECT {
			left: 0, right: 0, top: 0, bottom: 0,
		},
		fRestore: 0 as BOOL,
		fIncUpdate: 0 as BOOL,
		rgbReserved: [0 as BYTE, ..32],
	};

	let hdc = BeginPaint(hwnd, &mut ps);

	let old_obj = SelectObject(hdc, GetStockObject(DC_BRUSH));
	SelectObject(hdc, GetStockObject(DC_PEN));

	
	let old_brush_color = SetDCBrushColor(hdc, 0x00FFAAAA as COLORREF); //0x00bbggrr
	let old_pen_color   = SetDCPenColor  (hdc, 0x00FFDDDD as COLORREF); //0x00bbggrr 
	for &rect in canvas.rects.iter() {
		Rectangle(hdc, rect.lt.x as c_int, rect.lt.y as c_int, rect.rb.x as c_int, rect.rb.y as c_int);
	}
	SetDCBrushColor(hdc, old_brush_color);
	SetDCPenColor(hdc, old_pen_color);

	let old_bk_mode = SetBkMode(hdc, TRANSPARENT);
	for &(pos, ref block) in canvas.text_blocks.iter() {
		let text: Vec<u16> = block.text.as_slice().utf16_units().chain(Some(0).into_iter()).collect::<Vec<u16>>();
		TextOutW(hdc, pos.x as c_int, pos.y as c_int, text.as_ptr(), text.len() as c_int);
	}
	SetBkMode(hdc, old_bk_mode);

	SelectObject(hdc, old_obj);
	EndPaint(hwnd, &ps);
}

unsafe fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
	let ref mut buffer = *g_buffer;
	match msg {
		WM_CHAR => {
		    let ch32 = wparam as u32;
		    match ch32 {
		    	VK_UP     => append_up(buffer),
		    	VK_DOWN   => append_down(buffer),
		    	VK_LEFT   => append_left(buffer),
		    	VK_RIGHT  => append_right(buffer),
		    	VK_BACK   => append_backspace(buffer),
		    	VK_RETURN => append_newline(buffer),
		    	_         => match char::from_u32(ch32) {
					    		 Some(ch) => {
					    		     append_char(ch, buffer);		    			
					    		 },
					    	 	 None     => { },
					    	 },
		    }

			InvalidateRect(hwnd, 0 as *const RECT, 1 as BOOL);
			0
		},

		WM_PAINT => {		
			paint(hwnd, &stage(buffer));
			0
		},

		WM_DESTROY => {
			PostQuitMessage(0);
			0
		},

		_ => {
			DefWindowProcW(hwnd, msg, wparam, lparam)
		}
	}
}