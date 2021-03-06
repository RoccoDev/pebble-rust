/*
 * Copyright (c) 2019, Andrew Foote. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the copyright holder nor the
      names of its contributors may be used to endorse or promote products
      derived from this software without specific prior written permission.

 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
 * OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
#![allow(unused)]

use crate::pebble::internal::types::*;
use core::intrinsics;

use crate::pebble::internal::functions::declarations;

pub fn app_event_loop() {
    unsafe {
        declarations::app_event_loop();
    }
}

pub fn window_create() -> *mut Window {
    unsafe {
        declarations::window_create()
    }
}

pub fn window_destroy(window: *mut Window) {
    unsafe {
        declarations::window_destroy(window);
    }
}

pub fn window_set_click_config_provider<T>(window: *mut Window, func: extern fn(*mut T)) {
    unsafe {
        declarations::window_set_click_config_provider(window, intrinsics::transmute(func));
    }
}

pub fn window_set_click_config_provider_with_context<T>(window: *mut Window, func: extern fn(*mut T), ctx: *mut T) {
    unsafe {
        declarations::window_set_click_config_provider_with_context(window,
                                                                intrinsics::transmute(func),
                                                                ctx as *mut u8);
    }
}

pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers) {
    unsafe {
        declarations::window_set_window_handlers(window, handlers);
    }
}

pub fn window_set_background_color(window: *mut Window, color: GColor) {
    unsafe {
        declarations::window_set_background_color(window, color);
    }
}

pub fn window_set_user_data<T>(window: *mut Window, data: *mut T) {
    unsafe {
        declarations::window_set_user_data(window, data as *mut c_void);
    }
}

pub fn window_get_user_data<T>(window: *mut Window) -> *mut T {
    unsafe {
        declarations::window_get_user_data(window) as *mut T
    }
}

pub fn window_stack_push(window: *mut Window, animate: bool) {
    unsafe {
        if animate {
            declarations::window_stack_push(window, 1);
        } else {
            declarations::window_stack_push(window, 0);
        }
    }
}

pub fn window_get_root_layer(window: *mut Window) -> *mut Layer {
    unsafe {
        declarations::window_get_root_layer(window)
    }
}

pub fn window_single_click_subscribe<T>(button: u8, func: extern fn(*mut ClickRecognizer, *mut T)) {
    unsafe {
        declarations::window_single_click_subscribe(button, intrinsics::transmute(func));
    }
}

pub fn layer_create(bounds: GRect) -> *mut Layer {
    unsafe {
        declarations::layer_create(bounds)
    }
}

pub fn layer_destroy(layer: *mut Layer) {
    unsafe {
        declarations::layer_destroy(layer);
    }
}

pub fn layer_get_frame(layer: *mut Layer) -> GRect {
    unsafe {
        declarations::layer_get_frame(layer)
    }
}

pub fn layer_get_bounds(layer: *mut Layer) -> GRect {
    unsafe {
        declarations::layer_get_bounds(layer)
    }
}

pub fn layer_add_child(layer: *mut Layer, child: *mut Layer) {
    unsafe {
        declarations::layer_add_child(layer, child);
    }
}

pub fn layer_mark_dirty(layer: *mut Layer) {
    unsafe {
        declarations::layer_mark_dirty(layer);
    }
}

pub fn layer_set_update_proc(layer: *mut Layer, func: extern fn(*mut Layer, *mut GContext)) {
    unsafe {
        declarations::layer_set_update_proc(layer, func);
    }
}

pub fn text_layer_create(bounds: GRect) -> *mut TextLayer {
    unsafe {
        declarations::text_layer_create(bounds)
    }
}

pub fn text_layer_set_text(layer: *mut TextLayer, text: &str) {
    unsafe {
        declarations::text_layer_set_text(layer, text.as_ptr());
    }
}

pub fn text_layer_set_font(layer: *mut TextLayer, font: GFont) {
    unsafe {
        declarations::text_layer_set_font(layer, font);
    }
}

pub fn text_layer_get_layer(layer: *mut TextLayer) -> *mut Layer {
    unsafe {
        declarations::text_layer_get_layer(layer)
    }
}

pub fn gbitmap_create_with_resource(id: u32) -> *mut GBitmap {
    unsafe {
        declarations::gbitmap_create_with_resource(id)
    }
}

pub fn bitmap_layer_create(frame: GRect) -> *mut BitmapLayer {
    unsafe {
        declarations::bitmap_layer_create(frame)
    }
}

pub fn bitmap_layer_set_bitmap(layer: *mut BitmapLayer, bitmap: *mut GBitmap) {
    unsafe {
        declarations::bitmap_layer_set_bitmap(layer, bitmap);
    }
}

pub fn bitmap_layer_set_compositing_mode(layer: *mut BitmapLayer, mode: GCompOp) {
    unsafe {
        declarations::bitmap_layer_set_compositing_mode(layer, mode);
    }
}

pub fn bitmap_layer_get_layer(layer: *mut BitmapLayer) -> *mut Layer {
    unsafe {
        declarations::bitmap_layer_get_layer(layer)
    }
}

pub fn graphics_context_set_fill_color(ctx: *mut GContext, color: GColor) {
    unsafe {
        declarations::graphics_context_set_fill_color(ctx, color);
    }
}

pub fn graphics_fill_circle(ctx: *mut GContext, center: GPoint, radius: u16) {
    unsafe {
        declarations::graphics_fill_circle(ctx, center, radius);
    }
}

pub fn clock_is_24h_style() -> bool {
    unsafe {
        let response = declarations::clock_is_24h_style();
        response != 0
    }
}

pub fn tick_timer_service_subscribe(unit: TimeUnits, func: extern fn(*mut tm, TimeUnits)) {
    unsafe {
        declarations::tick_timer_service_subscribe(unit, func);
    }
}

pub fn time() -> usize {
    unsafe {
        declarations::time(core::ptr::null_mut())
    }
}

pub fn localtime(now: usize) -> *mut tm {
    unsafe {
        let now_ptr = &now as *const usize;
        declarations::localtime(now_ptr)
    }
}

pub fn gmtime(now: usize) -> *mut tm {
    unsafe {
        let now_ptr = &now as *const usize;
        declarations::gmtime(now_ptr)
    }
}

pub fn app_log(level: u8, msg: &str, name: &str) {
    unsafe {
        declarations::app_log(level, name.as_ptr(), 2,
                              msg.as_ptr());
    }
}