use std::fs;

use crate::models::CoreProps;

const ADDRESS_URL: &str = "C:/ProgramData/SteelSeries/SteelSeries Engine 3/coreProps.json";

pub fn get_props() -> Result<CoreProps, std::io::Error> {
    let file_contents = fs::read_to_string(ADDRESS_URL)?;
    let props = serde_json::from_str::<CoreProps>(&file_contents)?;

    Ok(props)
}
