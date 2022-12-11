mod mouse;
mod models;
mod gamesense;

use gamesense::GameSense;
use models::GameData;
use mouse::get_props;

use std::time::Duration;

use windows::core::GUID;
use windows::Win32::Media::Audio::{
    eConsole, eRender, Endpoints::IAudioMeterInformation, IMMDeviceEnumerator, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitialize, CLSCTX_ALL,
};

fn main() {
    let props = get_props()
        .expect("Error while getting adress");

    let gamesense = GameSense::new(
        "METER",
        "Meter",
        &props.address
    )
    .expect("error while registering game")
    .event("SYNC")
    .expect("error while registering SYNC event")
    .build();

    unsafe {
        CoInitialize(None).expect("Error calling CoInitialize");

        let x = &MMDeviceEnumerator as *const GUID;

        let instance: IMMDeviceEnumerator = CoCreateInstance(x, None, CLSCTX_ALL).unwrap();

        let default_audio_endpoint = instance
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .expect("Error while getting default audio endpoint");

        let meter_information = default_audio_endpoint
            .Activate::<IAudioMeterInformation>(CLSCTX_ALL, None)
            .expect("error getting information");

        loop {
            let Ok(peak_value) = meter_information.GetPeakValue() else {
                continue;
            };

            gamesense
                .send_event("SYNC", GameData {
                    value: peak_value * 200.0
                })
                .ok();

            std::thread::sleep(Duration::from_millis(50));
        }
    }
}
