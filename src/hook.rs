use winapi::um::winuser::{CallNextHookEx, GetMessageA, TranslateMessage,
    DispatchMessageA, SetWindowsHookExA, UnhookWindowsHookEx, MSLLHOOKSTRUCT};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::shared::windef::HHOOK;

pub fn call_hook(){
    let hook = set_mouse_hook();
    message_loop(hook);
    // unhook(hook);
}

fn message_loop(set_hook: HHOOK)-> Result<(), ()>{

    let mut msg = winapi::um::winuser::MSG {
        hwnd: std::ptr::null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: winapi::shared::windef::POINT { x: 0, y: 0 },
    };

    loop {
        let ret = unsafe{ GetMessageA(&mut msg, std::ptr::null_mut(), 0, 0) };

        if ret == 0 || ret == -1 {
            break;
        }

        unsafe{ TranslateMessage(&mut msg) };
        unsafe{ DispatchMessageA(&mut msg) };
    }

    Ok(())
}

fn set_mouse_hook()-> HHOOK{

    let hook = unsafe { SetWindowsHookExA(
        winapi::um::winuser::WH_MOUSE_LL,
        Some(hook_callback),
        GetModuleHandleA(std::ptr::null()),
        0,
    )};

    if hook.is_null(){
        panic!();
    }else{
        hook
    } 

}

pub fn unhook()-> Option<i32>{

    let hook = set_mouse_hook();

    let un_hook = unsafe{ UnhookWindowsHookEx(hook) };

    if un_hook == 0 {
        None
    }else{
        Some(un_hook)
    }

}

#[allow(dead_code)]
extern "system" fn hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {

    let event = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };

    match w_param {
        0x200 => {
            println!("Mouse moved to ({}, {})", event.pt.x, event.pt.y);
        }
        0x201 => {
            println!("Left mouse button pressed at ({}, {})", event.pt.x, event.pt.y);
        }
        0x202 => {
            println!("Left mouse button released at ({}, {})", event.pt.x, event.pt.y);
        }
        0x203 => {
            println!("Left mouse button double pressed at ({}, {})", event.pt.x, event.pt.y);
        }
        0x204 => {
            println!("Right mouse button pressed at ({}, {})", event.pt.x, event.pt.y);
        }
        0x205 => {
            println!("Right mouse button released at ({}, {})", event.pt.x, event.pt.y);
        }
        0x206 => {
            println!("Right mouse button double pressed at ({}, {})", event.pt.x, event.pt.y);
        }
        0x207 => {
            println!("Middle mouse button pressed at ({}, {})", event.pt.x, event.pt.y);
        }
        0x208 => {
            println!("Middle mouse button released at ({}, {})", event.pt.x, event.pt.y);
        }
        _ => {}
    }

    unsafe{ CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param) }
    
}