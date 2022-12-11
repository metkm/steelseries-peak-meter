mod mouse;
mod models;
mod gamesense;

use gamesense::GameSense;
use models::GameData;
use mouse::get_props;

use std::time::Duration;
use std::io::Write;

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

        let mut update_count = 0.0;
        let mut total = 0.0;
        let mut lock = std::io::stdout().lock();
        loop {
            let Ok(peak_value) = meter_information.GetPeakValue() else {
                continue;
            };

            let average_peak = total / update_count;

            gamesense
                .send_event("SYNC", GameData {
                    value: if peak_value > average_peak { 1 } else { 0 }
                })
                .ok();

            writeln!(lock, "Average {:.5} - Peak {:.5} - Update {}", average_peak, peak_value, update_count).ok();

            update_count += 1.0;
            total += peak_value;
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
