use crate::internal::error::{Error, ParseSection};

/// Parse policy used by `load_with_options`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParseMode {
    /// Reject any issue classified as an error in the current parser policy.
    #[default]
    Strict,
    /// Keep parsing when an issue is explicitly marked as safe-to-skip.
    Permissive,
}

/// Diagnostic severity emitted during parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

/// Coarse category for parse diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    UnknownBitFlags,
    UnexpectedReservedValue,
    TrailingBytes,
}

/// A non-fatal parse issue observed while decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub kind: DiagnosticKind,
    pub section: ParseSection,
    pub field: Option<&'static str>,
    pub offset: Option<usize>,
    pub message: String,
}

/// Parsed value plus any non-fatal diagnostics emitted during decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseReport<T> {
    pub value: T,
    pub diagnostics: Vec<Diagnostic>,
}

/// Options for `load_with_options`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LoadOptions {
    pub parse_mode: ParseMode,
}

impl LoadOptions {
    pub const fn strict() -> Self {
        Self {
            parse_mode: ParseMode::Strict,
        }
    }

    pub const fn permissive() -> Self {
        Self {
            parse_mode: ParseMode::Permissive,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ParseContext {
    mode: ParseMode,
    diagnostics: Vec<Diagnostic>,
}

impl ParseContext {
    pub(crate) fn new(mode: ParseMode) -> Self {
        Self {
            mode,
            diagnostics: Vec::new(),
        }
    }

    pub(crate) fn finish<T>(self, value: T) -> ParseReport<T> {
        ParseReport {
            value,
            diagnostics: self.diagnostics,
        }
    }

    pub(crate) fn warn(
        &mut self,
        kind: DiagnosticKind,
        section: ParseSection,
        field: Option<&'static str>,
        offset: Option<usize>,
        message: impl Into<String>,
        strict_error: Option<Error>,
    ) -> Result<(), Error> {
        match (self.mode, strict_error) {
            (ParseMode::Strict, Some(error)) => Err(error),
            _ => {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    kind,
                    section,
                    field,
                    offset,
                    message: message.into(),
                });
                Ok(())
            }
        }
    }
}
