use super::*;

#[repr(i32)]
pub enum Priority {
    High = TP_CALLBACK_PRIORITY_HIGH,
    Low = TP_CALLBACK_PRIORITY_LOW,
    Normal = TP_CALLBACK_PRIORITY_NORMAL,
}