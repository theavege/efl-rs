//! Error handling module for efl-rs.
//!
//! This module provides a comprehensive error handling system for the EFL bindings,
//! replacing `.unwrap()` calls with proper error propagation and custom error types.

use std::{
    ffi::{CString, NulError},
    fmt,
    ptr::NonNull,
};

/// Result type alias for EFL operations.
pub type EflResult<T> = Result<T, EflError>;

/// Main error type for EFL operations.
///
/// This enum represents all possible errors that can occur when using the EFL bindings.
#[derive(Debug)]
pub enum EflError {
    /// Error converting a string to CString (contains null byte).
    NullByte(NulError),
    
    /// Error when a widget pointer is null when it shouldn't be.
    NullWidget(&'static str),
    
    /// Error when a timer pointer is null when it shouldn't be.
    NullTimer,
    
    /// Error when a widget item pointer is null when it shouldn't be.
    NullWidgetItem,
    
    /// Error when an operation is performed on an empty/uninitialized widget.
    EmptyWidget(&'static str),
    
    /// Error when an index is out of bounds.
    IndexOutOfBounds {
        index: usize,
        length: usize,
    },
    
    /// Error when a value is outside the expected range.
    InvalidRange {
        min: f64,
        max: f64,
        value: f64,
    },
    
    /// Generic error message for other EFL-related errors.
    Other(String),
}

impl EflError {
    /// Create a NullByte error from a NulError.
    pub fn from_nul_error(err: NulError) -> Self {
        EflError::NullByte(err)
    }
    
    /// Create a NullWidget error with context.
    pub fn null_widget(context: &'static str) -> Self {
        EflError::NullWidget(context)
    }
    
    /// Create an EmptyWidget error with context.
    pub fn empty_widget(context: &'static str) -> Self {
        EflError::EmptyWidget(context)
    }
    
    /// Create an IndexOutOfBounds error.
    pub fn index_out_of_bounds(index: usize, length: usize) -> Self {
        EflError::IndexOutOfBounds { index, length }
    }
    
    /// Create an InvalidRange error.
    pub fn invalid_range(min: f64, max: f64, value: f64) -> Self {
        EflError::InvalidRange { min, max, value }
    }
}

impl fmt::Display for EflError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EflError::NullByte(err) => write!(f, "String contains null byte: {}", err),
            EflError::NullWidget(ctx) => write!(f, "Null widget pointer in {}", ctx),
            EflError::NullTimer => write!(f, "Null timer pointer"),
            EflError::NullWidgetItem => write!(f, "Null widget item pointer"),
            EflError::EmptyWidget(ctx) => write!(f, "Empty/uninitialized widget in {}", ctx),
            EflError::IndexOutOfBounds { index, length } => {
                write!(f, "Index {} out of bounds (length: {})", index, length)
            }
            EflError::InvalidRange { min, max, value } => {
                write!(f, "Value {} outside valid range [{}, {}]", value, min, max)
            }
            EflError::Other(msg) => write!(f, "EFL error: {}", msg),
        }
    }
}

impl std::error::Error for EflError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EflError::NullByte(err) => Some(err),
            _ => None,
        }
    }
}

impl From<NulError> for EflError {
    fn from(err: NulError) -> Self {
        EflError::NullByte(err)
    }
}

/// Helper trait for converting strings to CStrings with proper error handling.
pub trait CStringExt {
    /// Convert a string to CString, returning an EflResult.
    fn to_cstring(&self) -> EflResult<CString>;
    
    /// Convert a string to CString, panicking with a custom message on failure.
    /// 
    /// This should only be used in situations where the string is known to be valid
    /// (e.g., string literals without null bytes).
    fn expect_cstring(&self, context: &'static str) -> CString;
}

impl CStringExt for str {
    fn to_cstring(&self) -> EflResult<CString> {
        CString::new(self).map_err(EflError::from)
    }
    
    fn expect_cstring(&self, context: &'static str) -> CString {
        CString::new(self).unwrap_or_else(|_| {
            panic!("Failed to create CString in {}: string contains null byte", context)
        })
    }
}

impl CStringExt for String {
    fn to_cstring(&self) -> EflResult<CString> {
        CString::new(self.as_str()).map_err(EflError::from)
    }
    
    fn expect_cstring(&self, context: &'static str) -> CString {
        self.as_str().expect_cstring(context)
    }
}

/// Helper function to safely create a NonNull pointer.
pub fn nonnull_from_ptr<T>(ptr: *mut T, context: &'static str) -> EflResult<NonNull<T>> {
    NonNull::new(ptr).ok_or_else(|| EflError::null_widget(context))
}

/// Helper function to safely get a NonNull from Option.
pub fn nonnull_from_option<T>(opt: Option<NonNull<T>>, context: &'static str) -> EflResult<NonNull<T>> {
    opt.ok_or_else(|| EflError::empty_widget(context))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_cstring() {
        let result = "hello".to_cstring();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), "hello");
    }
    
    #[test]
    fn test_invalid_cstring() {
        let result = "hello\0world".to_cstring();
        assert!(result.is_err());
        match result {
            Err(EflError::NullByte(_)) => {},
            _ => panic!("Expected NullByte error"),
        }
    }
    
    #[test]
    fn test_index_out_of_bounds() {
        let err = EflError::index_out_of_bounds(5, 3);
        assert!(err.to_string().contains("5"));
        assert!(err.to_string().contains("3"));
    }
    
    #[test]
    fn test_invalid_range() {
        let err = EflError::invalid_range(0.0, 100.0, 150.0);
        assert!(err.to_string().contains("150"));
        assert!(err.to_string().contains("0"));
        assert!(err.to_string().contains("100"));
    }
    
    #[test]
    fn test_error_display() {
        let err = EflError::empty_widget("test context");
        assert!(err.to_string().contains("test context"));
    }
}
