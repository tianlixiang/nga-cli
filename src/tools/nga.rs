//! NGA CLI — the app itself, registered as a first-class tool so it
//! appears in the launchpad, tool picker, and can be spawned as a
//! nested terminal session (a non-AI "dumb" shell that can run the
//! upstream NGA CLI binary for side-by-side debugging / dogfooding).

use super::{HistoryShape, ToolDescriptor};

pub static DESCRIPTOR: ToolDescriptor = ToolDescriptor {
    id: "nga",
    display_name: "NGA CLI",
    binary_name: "nga",
    skill_dir_relative: None,
    has_hook_surface: false,
    history_shape: Some(HistoryShape::OpenCodeMixed {
        root_under_home: ".local/share/opencode/db",
    }),
    default_args: &[],
};
