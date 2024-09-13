use std::{io::Cursor, time::Duration};

use nt::{options::PubSubOptions, Instance, Publisher, Subscriber};
use result::PipelineResult;
use robotrs::time::get_time;

pub mod decode;
pub mod result;

pub struct Camera<'a> {
    result: Subscriber<'a, Vec<u8>>,
    driver_mode_pub: Publisher<'a, bool>,
    driver_mode_sub: Subscriber<'a, bool>,
    version: Subscriber<'a, String>,
    save_input_img_pub: Publisher<'a, i64>,
    save_output_img_pub: Publisher<'a, i64>,
    save_input_img_sub: Subscriber<'a, i64>,
    save_output_img_sub: Subscriber<'a, i64>,
    pipeline_idx_request: Publisher<'a, i64>,
    led_mode_request: Publisher<'a, i64>,
    pipeline_idx_state: Subscriber<'a, i64>,
    led_mode_state: Subscriber<'a, i64>,
    heartbeat: Subscriber<'a, i64>,
    camera_intrinsics: Subscriber<'a, Vec<f64>>,
    camera_distortion: Subscriber<'a, Vec<f64>>,
}

impl<'a> Camera<'a> {
    pub fn new(instance: &'a Instance, camera_name: &str) -> Self {
        let save_input_img =
            instance.topic(&format!("/photonvision/{}/inputSaveImgCmd", camera_name));
        let save_output_img =
            instance.topic(&format!("/photonvision/{}/outputSaveImgCmd", camera_name));

        Self {
            result: instance
                .topic(&format!("/photonvision/{}/rawBytes", camera_name))
                .subscribe_with_type_str(
                    PubSubOptions::default()
                        .periodic(Duration::from_millis(10))
                        .send_all(true),
                    "rawBytes",
                ),
            driver_mode_pub: instance
                .topic(&format!("/photonvision/{}/driverModeRequest", camera_name))
                .publish(Default::default()),
            driver_mode_sub: instance
                .topic(&format!("/photonvision/{}/driverMode", camera_name))
                .subscribe(Default::default()),
            save_input_img_pub: save_input_img.publish(Default::default()),
            save_input_img_sub: save_input_img.subscribe(Default::default()),
            save_output_img_pub: save_output_img.publish(Default::default()),
            save_output_img_sub: save_output_img.subscribe(Default::default()),
            pipeline_idx_request: instance
                .topic(&format!(
                    "/photonvision/{}/pipelineIndexRequest",
                    camera_name
                ))
                .publish(Default::default()),
            pipeline_idx_state: instance
                .topic(&format!("/photonvision/{}/pipelineIndexState", camera_name))
                .subscribe(Default::default()),
            heartbeat: instance
                .topic(&format!("/photonvision/{}/heartbeat", camera_name))
                .subscribe(Default::default()),
            camera_intrinsics: instance
                .topic(&format!("/photonvision/{}/cameraIntrinsics", camera_name))
                .subscribe(Default::default()),
            camera_distortion: instance
                .topic(&format!("/photonvision/{}/cameraDistortion", camera_name))
                .subscribe(Default::default()),

            led_mode_request: instance
                .topic("/photonvision/ledModeRequest")
                .publish(Default::default()),
            led_mode_state: instance
                .topic("/photonvision/ledModeState")
                .subscribe(Default::default()),
            version: instance
                .topic("/photonvision/version")
                .subscribe(Default::default()),
        }
    }

    pub fn print_raw(&self) {
        let data = self.result.get();

        for chunk in data.chunks(16) {
            for byte in chunk {
                print!("{:03} ", byte);
            }
            println!();
        }
    }

    pub fn get_result(&self) -> Result<PipelineResult, std::io::Error> {
        let (data, time) = self.result.get_with_time();

        let mut res = PipelineResult::decode(Cursor::new(data))?;

        res.set_timestamp((time as f64) / 1e6 - res.latency / 1e3);

        Ok(res)
    }
}
