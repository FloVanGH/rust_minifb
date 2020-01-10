#![cfg(feature = "utouch")]

use qt_core::*;
use qt_widgets::*;

use crate::buffer_helper;
use crate::error::Error;
use crate::key_handler::KeyHandler;
use crate::mouse_handler;
use crate::rate::UpdateRate;
use crate::InputCallback;
use crate::Result;
use crate::{CursorStyle, MouseButton, MouseMode};
use crate::{Key, KeyRepeat};
use crate::{MenuHandle, MenuItem, MenuItemHandle, UnixMenu, UnixMenuItem};
use crate::{Scale, WindowOptions};

use std::cmp;
use std::os::raw;
use std::thread;

pub struct Window {
    is_open: bool,
    is_active: bool,
    mouse_pos: Option<(i32, i32)>,
    mouse_scroll: Option<(i32, i32)>,
    /// The state of the left, middle and right mouse buttons
    mouse_state: (bool, bool, bool),
    update_rate: UpdateRate,
    buffer_width: usize,
    buffer_height: usize,
    window_scale: usize,
    key_handler: KeyHandler,
    menu_counter: MenuHandle,
    menus: Vec<UnixMenu>,
    qt_handle: thread::JoinHandle<()>,
}

impl Window {
    pub fn new(name: &str, width: usize, height: usize, opts: WindowOptions) -> Result<Window> {
        let qt_handle =
            thread::spawn(move || QApplication::init(|_| unsafe { QApplication::exec() }));

        let window = Window {
            is_open: true,
            is_active: true,
            mouse_pos: None,
            mouse_scroll: None,
            mouse_state: (false, false, false),
            key_handler: KeyHandler::new(),
            update_rate: UpdateRate::new(),
            buffer_width: width,
            buffer_height: height,
            window_scale: 1,
            menu_counter: MenuHandle(0),
            menus: Vec::new(),
            qt_handle,
        };

        Ok(window)
    }

    pub fn set_title(&mut self, title: &str) {}

    pub fn get_window_handle(&self) -> *mut raw::c_void {
        0 as *mut raw::c_void
    }

    pub fn set_background_color(&mut self, color: u32) {}

    pub fn update_with_buffer_stride(
        &mut self,
        buffer: &[u32],
        buf_width: usize,
        buf_height: usize,
        buf_stride: usize,
    ) -> Result<()> {
        Ok(())
    }

    pub fn update(&mut self) {}

    pub fn set_position(&mut self, x: isize, y: isize) {}

    pub fn get_size(&self) -> (usize, usize) {
        (0, 0)
    }

    pub fn get_scroll_wheel(&self) -> Option<(f32, f32)> {
        None
    }

    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        false
    }

    pub fn get_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        None
    }

    pub fn get_unscaled_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        None
    }

    pub fn set_cursor_style(&mut self, _cursor: CursorStyle) {
        // Orbital doesn't support cursor styles yet
    }

    #[inline]
    pub fn set_rate(&mut self, rate: Option<std::time::Duration>) {
        self.update_rate.set_rate(rate);
    }

    #[inline]
    pub fn update_rate(&mut self) {
        self.update_rate.update();
    }

    pub fn get_keys(&self) -> Option<Vec<Key>> {
        None
    }

    pub fn get_keys_pressed(&self, repeat: KeyRepeat) -> Option<Vec<Key>> {
        None
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        false
    }

    pub fn set_key_repeat_delay(&mut self, delay: f32) {}

    pub fn set_key_repeat_rate(&mut self, rate: f32) {}

    pub fn is_key_pressed(&self, key: Key, repeat: KeyRepeat) -> bool {
        false
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        false
    }

    pub fn set_input_callback(&mut self, callback: Box<InputCallback>) {}

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn is_active(&mut self) -> bool {
        self.is_active()
    }

    fn process_events(&mut self) {}

    /// Maps Orbital scancodes to MiniFB Key enums
    fn map_key_to_minifb(&self, scancode: u8) -> Option<Key> {
        None
    }

    /// Renders the given pixel data into the Orbital window
    fn render_buffer(&mut self, buffer: &[u32]) {}

    fn next_menu_handle(&mut self) -> MenuHandle {
        let handle = self.menu_counter;
        self.menu_counter.0 += 1;
        handle
    }

    pub fn add_menu(&mut self, menu: &Menu) -> MenuHandle {
        let handle = self.next_menu_handle();
        let mut menu = menu.internal.clone();
        menu.handle = handle;
        self.menus.push(menu);
        handle
    }

    pub fn get_unix_menus(&self) -> Option<&Vec<UnixMenu>> {
        Some(&self.menus)
    }

    pub fn remove_menu(&mut self, handle: MenuHandle) {
        self.menus.retain(|ref menu| menu.handle != handle);
    }

    pub fn is_menu_pressed(&mut self) -> Option<usize> {
        None
    }
}

pub struct Menu {
    pub internal: UnixMenu,
}

impl Menu {
    pub fn new(name: &str) -> Result<Menu> {
        Ok(Menu {
            internal: UnixMenu {
                handle: MenuHandle(0),
                item_counter: MenuItemHandle(0),
                name: name.to_owned(),
                items: Vec::new(),
            },
        })
    }

    pub fn add_sub_menu(&mut self, name: &str, sub_menu: &Menu) {
        let handle = self.next_item_handle();
        self.internal.items.push(UnixMenuItem {
            label: name.to_owned(),
            handle: handle,
            sub_menu: Some(Box::new(sub_menu.internal.clone())),
            id: 0,
            enabled: true,
            key: Key::Unknown,
            modifier: 0,
        });
    }

    fn next_item_handle(&mut self) -> MenuItemHandle {
        let handle = self.internal.item_counter;
        self.internal.item_counter.0 += 1;
        handle
    }

    pub fn add_menu_item(&mut self, item: &MenuItem) -> MenuItemHandle {
        let item_handle = self.next_item_handle();
        self.internal.items.push(UnixMenuItem {
            sub_menu: None,
            handle: self.internal.item_counter,
            id: item.id,
            label: item.label.clone(),
            enabled: item.enabled,
            key: item.key,
            modifier: item.modifier,
        });
        item_handle
    }

    pub fn remove_item(&mut self, handle: &MenuItemHandle) {
        self.internal
            .items
            .retain(|ref item| item.handle.0 != handle.0);
    }
}
