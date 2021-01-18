//! Safe wrapper for the `VIDIOC_EXPBUF` ioctl.
use bitflags::bitflags;
use std::mem;
use std::os::unix::io::{AsRawFd, FromRawFd};
use thiserror::Error;

use crate::{bindings, QueueType};

bitflags! {
    /// Flags that can be passed when exporting the buffer.
    pub struct ExpbufFlags: u32 {
        const CLOEXEC = libc::O_CLOEXEC as u32;
        const RDONLY = libc::O_RDONLY as u32;
        const WRONLY = libc::O_WRONLY as u32;
        const RDWR = libc::O_RDWR as u32;
    }
}

#[doc(hidden)]
mod ioctl {
    use crate::bindings::v4l2_exportbuffer;
    nix::ioctl_readwrite!(vidioc_expbuf, b'V', 16, v4l2_exportbuffer);
}

#[derive(Debug, Error)]
pub enum ExpbufError {
    #[error("Unexpected ioctl error: {0}")]
    IoctlError(#[from] nix::Error),
}

/// Safe wrapper around the `VIDIOC_EXPBUF` ioctl.
pub fn expbuf<F: AsRawFd, R: FromRawFd>(
    fd: &F,
    queue: QueueType,
    index: usize,
    plane: usize,
    flags: ExpbufFlags,
) -> Result<R, ExpbufError> {
    let mut v4l2_expbuf = bindings::v4l2_exportbuffer {
        type_: queue as u32,
        index: index as u32,
        plane: plane as u32,
        flags: flags.bits(),
        ..unsafe { mem::zeroed() }
    };

    unsafe { ioctl::vidioc_expbuf(fd.as_raw_fd(), &mut v4l2_expbuf) }?;

    Ok(unsafe { R::from_raw_fd(v4l2_expbuf.fd) })
}
