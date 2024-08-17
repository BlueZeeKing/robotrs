use std::{io::Cursor, time::Duration};

use nt::{options::PubSubOptions, Instance, Publisher, Subscriber};
use result::PipelineResult;
use robotrs::time::get_time;

pub mod decode;
pub mod result;

pub struct Camera {
    result: Subscriber<Vec<u8>>,
    driver_mode_pub: Publisher<bool>,
    driver_mode_sub: Subscriber<bool>,
    version: Subscriber<String>,
    save_input_img_pub: Publisher<i64>,
    save_output_img_pub: Publisher<i64>,
    save_input_img_sub: Subscriber<i64>,
    save_output_img_sub: Subscriber<i64>,
    pipeline_idx_request: Publisher<i64>,
    led_mode_request: Publisher<i64>,
    pipeline_idx_state: Subscriber<i64>,
    led_mode_state: Subscriber<i64>,
    heartbeat: Subscriber<i64>,
    camera_intrinsics: Subscriber<Vec<f64>>,
    camera_distortion: Subscriber<Vec<f64>>,
}

impl Camera {
    pub fn new(instance: Instance, camera_name: &str) -> Self {
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

    pub fn get_result(&self) -> Result<PipelineResult, std::io::Error> {
        let data = self.result.get();
        PipelineResult::decode(Cursor::new(data), get_time().as_micros() as i64)
    }
}
