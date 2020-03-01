/*!
 	An edit control is a rectangular control window to permit the user to enter and edit text by typing on the keyboard
*/
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM};
use winapi::um::winuser::{WS_VISIBLE, WS_DISABLED, ES_NUMBER, ES_LEFT, ES_CENTER, ES_RIGHT, ES_AUTOHSCROLL};
use crate::win32::window_helper as wh;
use crate::{Font, NwgError, HTextAlign, RawEventHandler};
use super::{ControlBase, ControlHandle};
use std::cell::RefCell;
use std::ops::Range;
use std::char;

const NOT_BOUND: &'static str = "TextInput is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TextInput handle is not HWND!";


bitflags! {
    pub struct TextInputFlags: u32 {
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const NUMBER = ES_NUMBER;
        const AUTO_SCROLL = ES_AUTOHSCROLL;
    }
}

/** 
An edit control is a rectangular control window to permit the user to enter and edit text by typing on the keyboard
This control only allow a single line input. For block of text, use `TextBox`.
Winapi documentation: https://docs.microsoft.com/en-us/windows/win32/controls/about-edit-controls#text-and-input-styles

TextInput is not behind any features.

**Builder parameters:**
  * `parent`:           **Required.** The text input parent container.
  * `text`:             The text input text.
  * `size`:             The text input size.
  * `position`:         The text input position.
  * `flags`:            A combination of the TextInputFlags values.
  * `font`:             The font used for the text input text
  * `limit`:            The maximum number of character that can be inserted in the control
  * `readonly`:         If the text input should allow user input or not
  * `password`:         The password character. If set to None, the textinput is a regular control.
  * `align`:            The alignment of the text in the text input
  * `background_color`: The color of the textinput top and bottom padding. This is not the white background under the text.

**Control events:**
  * `OnTextInput`: When a TextInput value is changed
  * `MousePress(_)`: Generic mouse press events on the button
  * `OnMouseMove`: Generic mouse mouse event

```rust
use native_windows_gui as nwg;
fn build_box(tbox: &mut nwg::TextInput, window: &nwg::Window, font: &nwg::Font) {
    nwg::TextInput::builder()
        .text("Hello")
        .font(Some(font))
        .parent(window)
        .build(tbox);
}
```
*/
#[derive(Default)]
pub struct TextInput {
    pub handle: ControlHandle,
    handler0: RefCell<Option<RawEventHandler>>,
}

impl TextInput {

    pub fn builder<'a>() -> TextInputBuilder<'a> {
        TextInputBuilder {
            text: "",
            size: (100, 25),
            position: (0, 0),
            flags: None,
            limit: 0,
            password: None,
            align: HTextAlign::Left,
            readonly: false,
            font: None,
            parent: None,
            background_color: None
        }
    }

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return the password character displayed by the text input. If the input is not a password, return None.
    pub fn password_char(&self) -> Option<char> {
        use winapi::um::winuser::EM_GETPASSWORDCHAR;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let raw_char = wh::send_message(handle, EM_GETPASSWORDCHAR as u32, 0, 0) as u32;
        match raw_char {
            0 => None,
            v => char::from_u32(v)
        }
    }

    /// Set or Remove the password character displayed by the text input.
    /// If the input is not a password all character are re-rendered with the new character
    pub fn set_password_char(&self, c: Option<char>) {
        use winapi::um::winuser::EM_SETPASSWORDCHAR;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_SETPASSWORDCHAR as u32, c.map(|c| c as usize).unwrap_or(0), 0);
    }

    /// Return the number of maximum character allowed in this text input
    pub fn limit(&self) -> u32 {
        use winapi::um::winuser::EM_GETLIMITTEXT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_GETLIMITTEXT as u32, 0, 0) as u32
    }

    /// Set the number of maximum character allowed in this text input
    /// If `limit` is 0, the text length is set to 0x7FFFFFFE characters 
    pub fn set_limit(&self, limit: usize) {
        use winapi::um::winuser::EM_SETLIMITTEXT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_SETLIMITTEXT as u32, limit, 0);
    }

    /// Check if the content of the text input was modified after it's creation
    pub fn modified(&self) -> bool {
        use winapi::um::winuser::EM_GETMODIFY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_GETMODIFY as u32, 0, 0) != 0
    }

    /// Manually set modified flag of the text input
    pub fn set_modified(&self, e: bool) {
        use winapi::um::winuser::EM_SETMODIFY;
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_SETMODIFY as u32, e as usize, 0);
    }

    /// Undo the last action by the user in the control
    pub fn undo(&self) {
        use winapi::um::winuser::EM_UNDO;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_UNDO as u32, 0, 0);
    }

    /// Return the selected range of characters by the user in the text input
    pub fn selection(&self) -> Range<u32> {
        use winapi::um::winuser::EM_GETSEL;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let mut start = 0u32;
        let mut end = 0u32;
        let ptr1 = &mut start as *mut u32;
        let ptr2 = &mut end as *mut u32;
        wh::send_message(handle, EM_GETSEL as UINT, ptr1 as WPARAM, ptr2 as LPARAM);

        start..end
    }

    /// Return the selected range of characters by the user in the text input
    pub fn set_selection(&self, r: Range<u32>) {
        use winapi::um::winuser::EM_SETSEL;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::send_message(handle, EM_SETSEL as u32, r.start as usize, r.end as isize);
    }

    /// Return the length of the user input in the control. This is better than test.len() as it
    /// does not allocate a string in memory
    pub fn len(&self) -> u32 {
        use winapi::um::winuser::EM_LINELENGTH;
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_LINELENGTH as u32, 0, 0) as u32
    }

    /// Return true if the TextInput value cannot be edited. Retrurn false otherwise.
    /// A user can still copy text from a readonly TextEdit (unlike disabled)
    pub fn readonly(&self) -> bool {
        use winapi::um::winuser::ES_READONLY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::get_style(handle) & ES_READONLY == ES_READONLY
    }

    /// Set the readonly flag of the text input
    /// A user can still copy text from a readonly TextEdit (unlike disabled)
    pub fn set_readonly(&self, r: bool) {
        use winapi::um::winuser::EM_SETREADONLY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::send_message(handle, EM_SETREADONLY as u32, r as WPARAM, 0);
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the text displayed in the TextInput
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the text displayed in the TextInput
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "EDIT"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_BORDER, WS_CHILD};
        
        WS_BORDER | ES_AUTOHSCROLL | WS_CHILD
    }

    /// Center the text vertically. Can't believe that must be manually hacked in.
    fn hook_non_client_size(&self, bg: Option<[u8; 3]>) {
        use crate::bind_raw_event_handler;
        use winapi::shared::windef::{HGDIOBJ, RECT, HBRUSH, POINT};
        use winapi::um::winuser::{WM_NCCALCSIZE, WM_NCPAINT, WM_SIZE, DT_CALCRECT, DT_LEFT, NCCALCSIZE_PARAMS, COLOR_WINDOW,};
        use winapi::um::winuser::{SWP_NOOWNERZORDER, SWP_NOSIZE, SWP_NOMOVE, SWP_FRAMECHANGED};
        use winapi::um::winuser::{GetDC, DrawTextW, ReleaseDC, GetClientRect, GetWindowRect, FillRect, ScreenToClient, SetWindowPos};
        use winapi::um::wingdi::{SelectObject, CreateSolidBrush, RGB};
        use std::{mem, ptr};

        if self.handle.blank() { panic!(NOT_BOUND); }
        self.handle.hwnd().expect(BAD_HANDLE);

        let brush = match bg {
            Some(c) => unsafe { CreateSolidBrush(RGB(c[0], c[1], c[2])) },
            None => COLOR_WINDOW as HBRUSH
        };

        unsafe {

        let handler = bind_raw_event_handler(&self.handle, 0, move |hwnd, msg, w, l| {
            match msg {
                WM_NCCALCSIZE  => {
                    if w == 0 { return None }

                    // Calculate client area height needed for a font
                    let font_handle = wh::get_window_font(hwnd);
                    let mut r: RECT = mem::zeroed();
                    let dc = GetDC(hwnd);
                    
                    let old = SelectObject(dc, font_handle as HGDIOBJ);
                    let calc: [u16;2] = [75, 121];
                    DrawTextW(dc, calc.as_ptr(), 2, &mut r, DT_CALCRECT | DT_LEFT);

                    let client_height = r.bottom;

                    SelectObject(dc, old);
                    ReleaseDC(hwnd, dc);

                    // Calculate NC area to center text.
                    let mut client: RECT = mem::zeroed();
                    let mut window: RECT = mem::zeroed();
                    GetClientRect(hwnd, &mut client);
                    GetWindowRect(hwnd, &mut window);

                    let window_height = window.bottom - window.top;
                    let center = ((window_height - client_height) / 2) - 4;
                    
                    // Save the info
                    let info_ptr: *mut NCCALCSIZE_PARAMS = l as *mut NCCALCSIZE_PARAMS;
                    let info = &mut *info_ptr;

                    info.rgrc[0].top += center;
                    info.rgrc[0].bottom -= center;
                },
                WM_NCPAINT  => {
                    let mut window: RECT = mem::zeroed();
                    let mut client: RECT = mem::zeroed();
                    GetWindowRect(hwnd, &mut window);
                    GetClientRect(hwnd, &mut client);

                    let mut pt1 = POINT {x: window.left, y: window.top};
                    ScreenToClient(hwnd, &mut pt1);

                    let mut pt2 = POINT {x: window.right, y: window.bottom};
                    ScreenToClient(hwnd, &mut pt2);

                    let top = RECT {
                        left: 0,
                        top: pt1.y,
                        right: client.right,
                        bottom: client.top
                    };

                    let bottom = RECT {
                        left: 0,
                        top: client.bottom,
                        right: client.right,
                        bottom: pt2.y
                    };

                    let dc = GetDC(hwnd);
                    FillRect(dc, &top, brush);
                    FillRect(dc, &bottom, brush);
                    ReleaseDC(hwnd, dc);
                },
                WM_SIZE => {
                    SetWindowPos(hwnd, ptr::null_mut(), 0, 0, 0, 0, SWP_NOOWNERZORDER | SWP_NOSIZE | SWP_NOMOVE | SWP_FRAMECHANGED);
                },
                _ => {}
            }

            None
        });

        *self.handler0.borrow_mut() = Some(handler);

        }
    }

}

impl Drop for TextInput {
    fn drop(&mut self) {
        use crate::unbind_raw_event_handler;
        
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            unbind_raw_event_handler(h);
        }
        
        self.handle.destroy();
    }
}

pub struct TextInputBuilder<'a> {
    text: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<TextInputFlags>,
    limit: usize,
    password: Option<char>,
    align: HTextAlign,
    readonly: bool,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>,
    background_color: Option<[u8; 3]>,
}

impl<'a> TextInputBuilder<'a> {

    pub fn flags(mut self, flags: TextInputFlags) -> TextInputBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn text(mut self, text: &'a str) -> TextInputBuilder<'a> {
        self.text = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> TextInputBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> TextInputBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn limit(mut self, limit: usize) -> TextInputBuilder<'a> {
        self.limit = limit;
        self
    }

    pub fn password(mut self, psw: Option<char>) -> TextInputBuilder<'a> {
        self.password = psw;
        self
    }

    pub fn align(mut self, align: HTextAlign) -> TextInputBuilder<'a> {
        self.align = align;
        self
    }

    pub fn readonly(mut self, read: bool) -> TextInputBuilder<'a> {
        self.readonly = read;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> TextInputBuilder<'a> {
        self.font = font;
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> TextInputBuilder<'a> {
        self.background_color = color;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TextInputBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut TextInput) -> Result<(), NwgError> {
        let mut flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        match self.align {
            HTextAlign::Left => flags |= ES_LEFT,
            HTextAlign::Center => flags |= ES_CENTER,
            HTextAlign::Right => {
                flags |= ES_RIGHT;
                flags &= !ES_AUTOHSCROLL;
            },
        }

        // Control must be hidden before the hook_non_client_size path
        // flags = flags & !WS_VISIBLE;

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("TextInput"))
        }?;

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .size(self.size)
            .position(self.position)
            .text(self.text)
            .parent(Some(parent))
            .build()?;

        out.hook_non_client_size(self.background_color);

        if self.limit > 0 {
            out.set_limit(self.limit);
        }

        if self.password.is_some() {
            out.set_password_char(self.password)
        }

        if self.readonly {
            out.set_readonly(self.readonly);
        }

        if self.font.is_some() {
            out.set_font(self.font);
        }

        Ok(())
    }

}

impl PartialEq for TextInput {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
