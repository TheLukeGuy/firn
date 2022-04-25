#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use firn::arch::x86;
use firn::arch::x86::device::{Cmos, DualPic};
use firn::arch::x86::{Cpu, Feature};
use firn::cpu::Restrict;
use firn::mem::{BasicMem, Eeprom, MemMap};
use firn::System;
use std::{mem, ptr, thread};
use windows::core::PCSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, DrawTextA, EndPaint, DT_CENTER, DT_SINGLELINE, DT_VCENTER, HBRUSH, PAINTSTRUCT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcA, DispatchMessageA, GetClientRect, GetMessageA, LoadCursorW,
    MessageBoxA, PostQuitMessage, RegisterClassExA, TranslateMessage, COLOR_WINDOW, CS_HREDRAW,
    CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MB_ICONEXCLAMATION, MB_OK, MSG, WM_DESTROY, WM_PAINT,
    WNDCLASSEXA, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

fn main() -> windows::core::Result<()> {
    if unsafe { create_window()? }.is_none() {
        return Ok(());
    }

    let mut sys = create_sys();
    thread::spawn(move || {
        sys.run();
    });

    let mut message = MSG::default();
    unsafe {
        while GetMessageA(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }
    }

    Ok(())
}

fn create_sys() -> System<Cpu> {
    let mem = BasicMem::new(640 * 1024);
    let eeprom = Eeprom::new_with_size(256 * 1024, x86::DEFAULT_BIOS);

    let mut map = MemMap::new(1024 * 1024);
    map.map_full(mem);
    map.map_from(0xc0000, 0xfffff, eeprom);

    let mut cpu = Cpu::new();
    cpu.add_feature(Feature::InstrCpu1);

    let pic = DualPic::new();
    let cmos = Cmos::new_current_time();

    let mut sys = System::new(cpu, map);
    sys.add_device(pic);
    sys.add_device(cmos);

    sys
}

unsafe fn create_window() -> windows::core::Result<Option<HWND>> {
    let instance = GetModuleHandleA(None);
    let window_class = WNDCLASSEXA {
        cbSize: mem::size_of::<WNDCLASSEXA>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        hInstance: instance,
        hCursor: LoadCursorW(None, IDC_ARROW)?,
        hbrBackground: HBRUSH((COLOR_WINDOW.0 as isize) + 1),
        lpszClassName: PCSTR(b"Window\0".as_ptr()),
        ..Default::default()
    };

    let value = RegisterClassExA(&window_class);
    if value == 0 {
        MessageBoxA(
            None,
            "Failed to register window!",
            "Error",
            MB_ICONEXCLAMATION | MB_OK,
        );
        return Ok(None);
    }

    let window = CreateWindowExA(
        Default::default(),
        "Window",
        "Firn",
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        800,
        600,
        None,
        None,
        instance,
        ptr::null(),
    );

    if window.0 == 0 {
        MessageBoxA(
            None,
            "Failed to create window!",
            "Error",
            MB_ICONEXCLAMATION | MB_OK,
        );
        return Ok(None);
    }

    Ok(Some(window))
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_PAINT => {
            let mut paint_struct = PAINTSTRUCT::default();
            let hdc = BeginPaint(window, &mut paint_struct);

            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);
            DrawTextA(
                hdc,
                "Waiting for the guest to initialize the display...".as_ref(),
                &mut rect,
                DT_SINGLELINE | DT_CENTER | DT_VCENTER,
            );

            EndPaint(window, &paint_struct);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => return DefWindowProcA(window, message, wparam, lparam),
    }

    LRESULT(0)
}
