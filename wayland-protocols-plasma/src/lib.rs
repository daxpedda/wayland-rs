//! This crate provides bindings to the plasma wayland protocol extensions
//! provided in <https://github.com/KDE/plasma-wayland-protocols>
//!
//! These bindings are built on top of the crates wayland-client and wayland-server.
//!
//! Each protocol module contains a `client` and a `server` submodules, for each side of the
//! protocol. The creation of these modules (and the dependency on the associated crate) is
//! controlled by the two cargo features `client` and `server`.

#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
mod protocol_macro;

pub mod appmenu {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/appmenu.xml",
        []
    );
}

pub mod blur {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/blur.xml",
        []
    );
}

pub mod contrast {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/contrast.xml",
        []
    );
}

pub mod dpms {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/dpms.xml",
        []
    );
}

pub mod fake_input {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/fake-input.xml",
        []
    );
}

pub mod idle {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/idle.xml",
        []
    );
}

pub mod keystate {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/keystate.xml",
        []
    );
}

pub mod output_device {
    pub mod v1 {
        wayland_protocol!(
            "./plasma-wayland-protocols/src/protocols/outputdevice.xml",
            []
        );
    }

    pub mod v2 {
        wayland_protocol!(
            "./plasma-wayland-protocols/src/protocols/kde-output-device-v2.xml",
            []
        );
    }
}

pub mod output_management {
    pub mod v1 {
        wayland_protocol!(
            "./plasma-wayland-protocols/src/protocols/output-management.xml",
            [crate::output_device::v1]
        );
    }

    pub mod v2 {
        wayland_protocol!(
            "./plasma-wayland-protocols/src/protocols/kde-output-management-v2.xml",
            [crate::output_device::v2]
        );
    }
}


pub mod primary_output {
    pub mod v1 {
        wayland_protocol!(
            "./plasma-wayland-protocols/src/protocols/kde-primary-output-v1.xml",
            []
        );
    }
}

pub mod plasma_shell {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/plasma-shell.xml",
        []
    );
}

pub mod plasma_virtual_desktop {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/plasma-virtual-desktop.xml",
        []
    );
}

// This protocol is disabled for now as the XML file contains stuff wayland-scanner cannot parse. 
//
// pub mod plasma_window_management {
//     wayland_protocol!(
//         "./plasma-wayland-protocols/src/protocols/plasma-window-management.xml",
//         []
//     );
// }


pub mod screencast {
    pub mod v1 {
        wayland_protocol!(
            "./plasma-wayland-protocols/src/protocols/screencast.xml",
            []
        );
    }
}

pub mod server_decoration_palette {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/server-decoration-palette.xml",
        []
    );
}

pub mod server_decoration {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/server-decoration.xml",
        []
    );
}

pub mod shadow {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/shadow.xml",
        []
    );
}

pub mod slide {
    wayland_protocol!(
        "./plasma-wayland-protocols/src/protocols/slide.xml",
        []
    );
}