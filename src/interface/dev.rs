//! Interaction interface with the acutal ublk device

use compio::fs::File;
use compio::io::AsyncWriteAt;
use std::os::fd::{FromRawFd, IntoRawFd};
use std::path::Path;
use std::u32;

use crate::interface::structs;

const UBLK_DEV_NODE: &str = "/dev/ublk-control";

/// Ublk device node
#[derive(Clone)]
struct ControlDevice {
    file: File,
}

impl ControlDevice {
    pub fn new() -> std::io::Result<Self> {
        tracing::info!(UBLK_DEV_NODE, "opening control file node");
        Self::open(Path::new(UBLK_DEV_NODE))
    }

    /// Opens a compio file from a raw file descriptor conversion
    fn open(path: &Path) -> std::io::Result<Self> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;
        let fd = file.into_raw_fd();

        // SAFETY: into_raw_fd transfers sole ownership of this open fd.
        let file = unsafe { File::from_raw_fd(fd) };

        Ok(Self { file })
    }

    /// Instantiates a new user device
    async fn add(&self, nr_hw_queues: u16, queue_depth: u16, max_io_buf_bytes: u32) {
        let msg = structs::UblkSrvCtrlDevInfo {
            nr_hw_queues.min(1),
            queue_depth.min(1),
            max_io_buf_bytes.min(1 << 20),
            dev_id: u32::MAX, // kernel decides for us
            ..Default::default()
        };
    }

    /// Stops request handling for the user device
    async fn stop() {
        unimplemented!()
    }

    /// Removes the user device id to make it allocatable
    async fn remove() {
        unimplemented!()
    }

    /// Get the raw control file
    fn as_file(&self) -> &File {
        &self.file
    }

    /// Get the raw mutable control file
    fn as_mut_file(&mut self) -> &mut File {
        &mut self.file
    }
}

#[cfg(test)]
mod tests {
    use super::ControlDevice;
    use compio::buf::BufResult;
    use compio::io::{AsyncReadAtExt, AsyncWriteAtExt};
    use std::io::ErrorKind;

    #[compio::test]
    async fn opens_existing_file_for_reading_and_writing() {
        let dir = tempfile::tempdir().expect("test directory should be created");
        let path = dir.path().join("control");
        std::fs::write(&path, b"before").expect("control file should contain the initial data");

        let mut device = ControlDevice::open(&path).expect("existing control file should open");
        let BufResult(result, contents) = device
            .as_file()
            .read_exact_at(Vec::with_capacity(6), 0)
            .await;
        result.expect("control file should remain readable after ownership transfer");
        assert_eq!(contents, b"before");

        device
            .as_mut_file()
            .write_all_at(b"after!", 0)
            .await
            .0
            .expect("control file should be writable through the mutable accessor");
        assert_eq!(
            std::fs::read(&path).expect("updated control file should remain readable"),
            b"after!"
        );
    }

    #[compio::test]
    async fn clone_remains_usable_after_original_is_dropped() {
        let dir = tempfile::tempdir().expect("test directory should be created");
        let path = dir.path().join("control");
        std::fs::write(&path, b"shared").expect("control file should contain the shared data");

        let device = ControlDevice::open(&path).expect("existing control file should open");
        let clone = device.clone();
        drop(device);

        let BufResult(result, contents) = clone
            .as_file()
            .read_exact_at(Vec::with_capacity(6), 0)
            .await;
        result.expect("clone should remain readable after the original device is dropped");
        assert_eq!(contents, b"shared");
    }

    #[test]
    fn missing_file_returns_not_found_without_creating_it() {
        let dir = tempfile::tempdir().expect("test directory should be created");
        let path = dir.path().join("missing");

        let error = ControlDevice::open(&path)
            .err()
            .expect("opening a missing control file should fail");

        assert_eq!(error.kind(), ErrorKind::NotFound);
        assert!(!path.exists());
    }

    #[test]
    fn opening_directory_returns_an_error() {
        let dir = tempfile::tempdir().expect("test directory should be created");

        let error = ControlDevice::open(dir.path())
            .err()
            .expect("opening a directory as a control file should fail");

        assert_eq!(error.kind(), ErrorKind::IsADirectory);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn opens_ublk_control_device() {
        use std::os::fd::AsFd;

        let device = ControlDevice::new().expect("ublk control device should open");
        device
            .as_file()
            .as_fd()
            .try_clone_to_owned()
            .expect("ublk control descriptor should be valid and duplicable");
    }
}
