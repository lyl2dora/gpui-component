mod blink_cursor;
mod change;
mod clear_button;
mod cursor;
mod display_map;
mod element;
mod indent;
mod input;
mod lsp;
mod mask_pattern;
mod mode;
mod movement;
mod number_input;
mod otp_input;
pub(crate) mod popovers;
mod rope_ext;
mod search;
mod selection;
mod state;

pub(crate) use clear_button::*;
pub use cursor::*;
#[cfg(target_family = "wasm")]
pub use display_map::folding::Tree;
pub use display_map::{BufferPoint, DisplayMap, DisplayPoint, FoldRange};
pub use indent::TabSize;
pub use input::*;
pub use lsp::*;
pub use lsp_types::Position;
pub use mask_pattern::MaskPattern;
pub use number_input::{NumberInput, NumberInputEvent, StepAction};
pub use otp_input::*;
pub use rope_ext::{InputEdit, Point, RopeExt, RopeLines};
pub use ropey::Rope;
pub use state::*;

/// 对 String 的底层字节逐字节写零，防止编译器优化掉清零操作。
/// 用于密码输入框 drop 时清除敏感数据。
pub(super) fn zeroize_string(s: &mut String) {
    unsafe {
        let bytes = s.as_mut_str().as_bytes_mut();
        for byte in bytes.iter_mut() {
            std::ptr::write_volatile(byte as *mut u8, 0);
        }
    }
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    s.clear();
}
