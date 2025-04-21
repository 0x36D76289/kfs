use crate::screen::Screen;
use crate::vga::{ColorCode, Color};
use crate::printk::init_printk;
use core::sync::atomic::{AtomicUsize, Ordering};

pub const MAX_SCREENS: usize = 4;

static ACTIVE_SCREEN: AtomicUsize = AtomicUsize::new(0);

static mut SCREEN_INITIALIZED: [bool; MAX_SCREENS] = [false, false, false, false];

static mut SCREENS: [Option<Screen>; MAX_SCREENS] = [None, None, None, None];

pub fn init_screens() {
    unsafe {
        SCREENS[0] = Some(Screen::new(ColorCode::new(Color::White, Color::Black)));
        
        SCREENS[1] = Some(Screen::new(ColorCode::new(Color::Green, Color::Black)));
        
        SCREENS[2] = Some(Screen::new(ColorCode::new(Color::Cyan, Color::Blue)));
        
        SCREENS[3] = Some(Screen::new(ColorCode::new(Color::Black, Color::LightGray)));
        
        SCREEN_INITIALIZED[0] = true;
        
        if let Some(ref mut screen) = SCREENS[0] {
            screen.activate();
            init_printk(screen as *mut Screen);
        }
    }
}

// Changer d'écran
pub fn switch_to_screen(screen_idx: usize) -> bool {
    if screen_idx >= MAX_SCREENS {
        return false;
    }
    
    let current_idx = ACTIVE_SCREEN.load(Ordering::SeqCst);
    
    if current_idx == screen_idx {
        return true;
    }
    
    unsafe {
        if let Some(ref mut screen) = SCREENS[current_idx] {
            screen.deactivate();
        }
        
        if let Some(ref mut screen) = SCREENS[screen_idx] {
            screen.activate();
            
            init_printk(screen as *mut Screen);
            
            ACTIVE_SCREEN.store(screen_idx, Ordering::SeqCst);
            
            let already_initialized = SCREEN_INITIALIZED[screen_idx];
            if !already_initialized {
                SCREEN_INITIALIZED[screen_idx] = true;
            }
            
            return !already_initialized;
        }
    }
    
    false
}

pub fn get_active_screen() -> usize {
    ACTIVE_SCREEN.load(Ordering::SeqCst)
}

pub unsafe fn get_active_screen_ref() -> Option<&'static mut Screen> {
    let idx = get_active_screen();
    unsafe {
        if let Some(ref mut screen) = SCREENS[idx] {
            Some(screen)
        } else {
            None
        }
    }
}
