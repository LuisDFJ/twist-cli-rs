use nokhwa::pixel_format::RgbFormat;
use nokhwa::{Camera, query};
use nokhwa::utils::{ApiBackend, CameraInfo, RequestedFormat, RequestedFormatType};

use std::path;
use std::io::{self,Error,ErrorKind};
use std::time;

pub struct CameraParams {
    name  : String,
    timer : time::Duration,
}

impl CameraParams {
    pub fn new( name : &str, timer : u64 ) -> Self {
        CameraParams { name: name.to_string(), timer : time::Duration::from_millis(timer) }
    }
}

pub struct CameraCapture {
    dir   : path::PathBuf,
    camera : Camera,
    pub info   : CameraInfo,
    timer  : (time::Instant, time::Duration),
    pub count  : u32,
}

use std::fs;
impl CameraCapture {
    pub fn new( params : &CameraParams, dir : &path::PathBuf ) -> io::Result<Self> {
        let dir = dir.join("cam/");
        fs::create_dir_all(dir.clone())
            .map_err(|_| Error::new(ErrorKind::Other, "fail to create target dir"))?;
        let mut devices = query(ApiBackend::Auto)
            .map_err(|_| Error::new(ErrorKind::Other, "fail to query devices"))?;
        devices.sort_by_key(|d| d.index().as_index().unwrap_or(0));
        let dev_names = devices.iter().fold("".to_string(), |a,b| a + "  >" + &b.human_name() + "\n" );
        let info = devices
            .into_iter().find(|dev| dev.human_name().contains(&params.name) )
            .ok_or(Error::new(ErrorKind::Other, "fail to find device: \n".to_string() + &dev_names))?;
        let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);
        let mut camera = Camera::new( info.index().clone(), format)
            .map_err(|_| Error::new(ErrorKind::Other, "fail to create camera"))?;
        camera.open_stream()
            .map_err(|_| Error::new(ErrorKind::Other, "fail to open stream"))?;
        Ok( CameraCapture { 
            dir,
            camera,
            info,
            timer: (time::Instant::now(),params.timer),
            count : 0
        } )
    }
    pub fn capture( self : &mut Self, speed : f64 ) -> io::Result<()> {
        if self.timer.0.elapsed() > self.timer.1 {
            self.timer.0 = time::Instant::now();
            self.count += 1;
            let frame = self.camera.frame()
                .map_err(|_| Error::new(ErrorKind::Other, "fail to capture frame"))?;
            let decoded = frame.decode_image::<RgbFormat>()
                .map_err(|_| Error::new(ErrorKind::Other, "fail to decode frame"))?;
            let distance = self.count as f64 * self.timer.1.as_secs_f64() * speed;
            let distance = format!("{:08.2}", distance).replace(".", "_");
            let filename = format!("capture_deg_{}.png", distance );
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
