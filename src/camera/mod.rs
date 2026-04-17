use nokhwa::pixel_format::RgbFormat;
use nokhwa::{Camera, query};
use nokhwa::utils::{ApiBackend, CameraInfo, RequestedFormat, RequestedFormatType};

use std::path;
use std::io::{self,Error,ErrorKind};
use std::time;

#[allow(unused)]
pub struct CameraCapture {
    dir   : path::PathBuf,
    camera : Camera,
    info   : CameraInfo,
    timer  : (time::Instant, time::Duration),
    pub count  : u32,
}

impl CameraCapture {
    pub fn new( name : &str, t : u64, dir : &str ) -> io::Result<Self> {
        let mut devices = query(ApiBackend::Auto)
            .map_err(|_| Error::new(ErrorKind::Other, "fail to query devices"))?;
        devices.sort_by_key(|d| d.index().as_index().unwrap_or(0));
        let info = devices
            .into_iter().find(|dev| dev.human_name().contains(name) )
            .ok_or(Error::new(ErrorKind::Other, "fail to find device"))?;
        let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);
        let mut camera = Camera::new( info.index().clone(), format)
            .map_err(|_| Error::new(ErrorKind::Other, "fail to create camera"))?;
        camera.open_stream()
            .map_err(|_| Error::new(ErrorKind::Other, "fail to open stream"))?;
        Ok( CameraCapture { 
            dir : path::PathBuf::from(dir),
            camera,
            info,
            timer: (time::Instant::now(),time::Duration::from_millis(t)),
            count : 0
        } )
    }
    pub fn capture( self : &mut Self ) -> io::Result<()> {
        if self.timer.0.elapsed() > self.timer.1 {
            self.timer.0 = time::Instant::now();
            self.count += 1;
            let frame = self.camera.frame()
                .map_err(|_| Error::new(ErrorKind::Other, "fail to capture frame"))?;
            let decoded = frame.decode_image::<RgbFormat>()
                .map_err(|_| Error::new(ErrorKind::Other, "fail to decode frame"))?;
            let filename = format!("capture_t_{}.png", self.count as u128 * self.timer.1.as_millis());
            decoded.save( self.dir.join(filename) )
                .map_err(|_| Error::new(ErrorKind::Other, "fail to save decoded frame"))?;
        }
        Ok(())
    }
}

impl Drop for CameraCapture {
    fn drop(&mut self) {
        let _ = self.camera.stop_stream();
    }
}
