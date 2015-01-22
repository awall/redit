#![feature(globs)]

use std::char;
use std::mem::transmute;

use win32::*;
use geometry::*;
use buffer::*;

pub mod win32;
mod geometry;
mod buffer;

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
				y: 0,
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
	let rects: Vec<Rect> = vec![
		Rect { 
			lt: Pos { x:  curx    * 8, y:  cury    * 14 }, 
			rb: Pos { x: (curx+1) * 8, y: (cury+1) * 14 },
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
	
	let front_hdc = BeginPaint(hwnd, &mut ps);

	let width = ps.rcPaint.right - ps.rcPaint.left;
	let height = ps.rcPaint.bottom - ps.rcPaint.top;
	let back_hdc = CreateCompatibleDC(front_hdc);
	let bmp = CreateCompatibleBitmap(front_hdc, width, height);
	let old_bmp = SelectObject(back_hdc, bmp);

	let old_fg = SelectObject(back_hdc, GetStockObject(NULL_PEN));
	let old_bg = SelectObject(back_hdc, GetStockObject(WHITE_BRUSH));
	Rectangle(back_hdc, 0, 0, width + 1, height + 1);
	SelectObject(back_hdc, old_bg);
	SelectObject(back_hdc, old_fg);

	let old_brush = SelectObject(back_hdc, GetStockObject(DC_BRUSH));
	let old_pen   = SelectObject(back_hdc, GetStockObject(DC_PEN));
	let old_font  = SelectObject(back_hdc, GetStockObject(ANSI_FIXED_FONT));
	
	let old_brush_color = SetDCBrushColor(back_hdc, 0x00FFAAAA as COLORREF); //0x00bbggrr
	let old_pen_color   = SetDCPenColor  (back_hdc, 0x00FFDDDD as COLORREF); //0x00bbggrr 
	for &rect in canvas.rects.iter() {
		Rectangle(back_hdc, rect.lt.x as c_int, rect.lt.y as c_int, rect.rb.x as c_int, rect.rb.y as c_int);
	}
	SetDCBrushColor(back_hdc, old_brush_color);
	SetDCPenColor(back_hdc, old_pen_color);

	let old_bk_mode = SetBkMode(back_hdc, TRANSPARENT);
	for &(pos, ref block) in canvas.text_blocks.iter() {
		let text: Vec<u16> = block.text.as_slice().utf16_units().chain(Some(0).into_iter()).collect::<Vec<u16>>();
		TextOutW(back_hdc, pos.x as c_int, pos.y as c_int, text.as_ptr(), text.len() as c_int);
	}
	SetBkMode(back_hdc, old_bk_mode);

	BitBlt(front_hdc, 0, 0, width, height, back_hdc, 0, 0, SRCCOPY);

	SelectObject(back_hdc, old_font);
	SelectObject(back_hdc, old_pen);
	SelectObject(back_hdc, old_brush);
	SelectObject(back_hdc, old_bmp);
	DeleteObject(bmp);
	DeleteDC(back_hdc);

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

		WM_ERASEBKGND => {
			1
		},

		_ => {
			DefWindowProcW(hwnd, msg, wparam, lparam)
		}
	}
}