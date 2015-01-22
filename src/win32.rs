extern crate libc;

pub use self::libc::types::os::arch::c95::{ 
	c_int, c_uint, c_long, c_ulong, c_ushort, c_char, wchar_t, 
	c_short, c_uchar, c_schar, c_float, c_double,
}; 

pub use self::libc::types::os::arch::c99::{ 
	c_longlong, c_ulonglong 
};

pub use self::libc::types::common::c95::{
	c_void,
};

pub type BOOL = c_int;
pub type BYTE = u8;
pub type CHAR = c_char;
pub type DWORD = c_ulong;
pub type LONG = c_long;
pub type UINT = c_uint;
pub type WCHAR = wchar_t;
pub type WORD = c_ushort;

#[cfg(target_arch = "x86")]    #[allow(non_camel_case_types)] pub type LONG_PTR = c_long;
#[cfg(target_arch = "x86_64")] #[allow(non_camel_case_types)] pub type LONG_PTR = i64;
#[cfg(target_arch = "x86")]    #[allow(non_camel_case_types)] pub type UINT_PTR = c_uint;
#[cfg(target_arch = "x86_64")] #[allow(non_camel_case_types)] pub type UINT_PTR = u64;

pub type PVOID = *mut c_void;
pub type LPCWSTR = *const WCHAR;
pub type LPSTR = *mut CHAR;
pub type LPVOID = *mut c_void;

pub type ATOM = WORD;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type WPARAM = UINT_PTR;
pub type WNDPROC = *const c_void;

pub type HANDLE = PVOID;
pub type HFONT = HANDLE;
pub type HBITMAP = HANDLE;
pub type HBRUSH = HANDLE;
pub type HCURSOR = HANDLE;
pub type HDC = HANDLE;
pub type HGDIOBJ = HANDLE;
pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMENU = HANDLE;
pub type HMODULE = HANDLE;
pub type HWND = HANDLE;

pub type COLORREF = DWORD;

pub static WS_BORDER: u32 = 0x800000;
pub static WS_CAPTION: u32 = 0xc00000;
pub static WS_CHILD: u32 = 0x40000000;
pub static WS_CHILDWINDOW: u32 = 0x40000000;
pub static WS_CLIPCHILDREN: u32 = 0x2000000;
pub static WS_CLIPSIBLINGS: u32 = 0x4000000;
pub static WS_DISABLED: u32 = 0x8000000;
pub static WS_DLGFRAME: u32 = 0x400000;
pub static WS_GROUP: u32 = 0x20000;
pub static WS_HSCROLL: u32 = 0x100000;
pub static WS_ICONIC: u32 = 0x20000000;
pub static WS_MAXIMIZE: u32 = 0x1000000;
pub static WS_MAXIMIZEBOX: u32 = 0x10000;
pub static WS_MINIMIZE: u32 = 0x20000000;
pub static WS_MINIMIZEBOX: u32 = 0x20000;
pub static WS_OVERLAPPED: u32 = 0;
pub static WS_OVERLAPPEDWINDOW: u32 = 0xcf0000;
pub static WS_POPUP: u32 = 0x80000000;
pub static WS_POPUPWINDOW: u32 = 0x80880000;
pub static WS_SIZEBOX: u32 = 0x40000;
pub static WS_SYSMENU: u32 = 0x80000;
pub static WS_TABSTOP: u32 = 0x10000;
pub static WS_THICKFRAME: u32 = 0x40000;
pub static WS_TILED: u32 = 0;
pub static WS_TILEDWINDOW: u32 = 0xcf0000;
pub static WS_VISIBLE: u32 = 0x10000000;
pub static WS_VSCROLL: u32 = 0x200000;

pub const WM_CHAR: UINT = 0x0102;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_PAINT: UINT = 0x000F;
pub const WM_ERASEBKGND: UINT = 0x0014;

pub const VK_RETURN: UINT = 0x000D;
pub const VK_BACK:   UINT = 0x0008;
pub const VK_LEFT:   UINT = 0x0025;
pub const VK_UP:     UINT = 0x0026;
pub const VK_RIGHT:  UINT = 0x0027;
pub const VK_DOWN:   UINT = 0x0028;

pub const TRANSPARENT: c_int = 1;
pub const OPAQUE:      c_int = 2;
pub const SRCCOPY:     DWORD = 0xcc0020;

pub const WHITE_BRUSH: c_int = 0;
pub const NULL_PEN: c_int = 8;
pub const ANSI_FIXED_FONT: c_int = 11;
pub const DC_BRUSH: c_int = 18;
pub const DC_PEN:   c_int = 19;
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
pub struct WNDCLASSEX {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
    pub hIconSm: HICON,
}

#[allow (non_snake_case)]
#[repr(C)]
pub struct MSG {
   pub hwnd: HWND,
   pub message: UINT,
   pub wParam: WPARAM,
   pub lParam: LPARAM,
   pub time: DWORD,
   pub pt: POINT,
}

#[allow (non_snake_case)]
#[repr(C)]
pub struct PAINTSTRUCT {
   pub hdc: HDC,
   pub fErase: BOOL,
   pub rcPaint: RECT,
   pub fRestore: BOOL,
   pub fIncUpdate: BOOL,
   pub rgbReserved: [BYTE, ..32],
}

#[repr(C)]
pub struct POINT {
	pub x: LONG,
	pub y: LONG,
}

#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

#[link (name="kernel32")]
extern "system" {
	pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

#[link (name="user32")]
extern "system" {
    pub fn CreateWindowExW(
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

	pub fn GetMessageW(
		lpMsg: *const MSG,
		hWnd: HWND,
		wMsgFilterMin: UINT,
		wMsgFilterMAx: UINT,
	) -> BOOL;

	pub fn DefWindowProcW(
         hWnd: HWND,
         Msg: UINT,
         wParam: WPARAM,
         lParam: LPARAM,
	) -> LRESULT;

	pub fn RegisterClassExW(lpwcx: *const WNDCLASSEX) -> ATOM;
    pub fn ShowWindow(hwnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn UpdateWindow(hwnd: HWND) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
	pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
	pub fn PostQuitMessage(nExitCode: c_int);
	pub fn BeginPaint(hwnd: HWND, lpPaint: *mut PAINTSTRUCT) -> HDC;
	pub fn EndPaint(hwnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
	pub fn InvalidateRect(hwnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
	pub fn GetStockObject(fnObject: c_int) -> HGDIOBJ;
}

#[link (name="gdi32")]
extern "system" {
	pub fn TextOutW(
  		hdc: HDC,
  		nXStart: c_int,
  		nYStart: c_int,
  	    lpString: LPCWSTR,
  	    cchString: c_int,
	) -> BOOL;

	pub fn Rectangle(
		hdc: HDC,
		left: c_int,
		top: c_int,
		right: c_int,
		bottom: c_int,
	) -> BOOL;

	pub fn SetBkMode(
		hdc: HDC,
		iBkMode: c_int,
	) -> c_int;

	pub fn SetDCBrushColor(
  		hdc: HDC,
  		crColor: COLORREF,
	) -> COLORREF;

	pub fn SetDCPenColor(
  		hdc: HDC,
  		crColor: COLORREF,
	) -> COLORREF;

	pub fn SelectObject(
		hdc: HDC,
		hgdiObj: HGDIOBJ,
	) -> HGDIOBJ;

	pub fn DeleteObject(
		hgdiObj: HGDIOBJ,
	) -> BOOL;

	pub fn DeleteDC(
		hdc: HDC,
	) -> BOOL;

	pub fn CreateCompatibleDC(
		hdc: HDC,
	) -> HDC;

	pub fn CreateCompatibleBitmap(
  		hdc: HDC,
  		nWidth: c_int,
  		nHeight: c_int,
	) -> HBITMAP;

	pub fn BitBlt(
	  hdcDest: HDC,
	  nXDest: c_int,
	  nYDest: c_int,
	  nWidth: c_int,
	  nHeight: c_int,
	  hdcSrc: HDC,
	  nXSrc: c_int,
	  nYSrc: c_int,
	  dwRop: DWORD,
	) -> BOOL;
}