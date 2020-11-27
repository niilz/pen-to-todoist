// Auth is done through JWT tokens!
// Those were the steps to enable Vision API:
// - Create account
// - Create project
// - Download credentials json
// - Set GOOGLE_APPLICATION_CREDENTIALS env var to access the json-file
// - Install gcloud client
// - Enable Vision API on google cloud platform
// - Activate Vision API for your project using the gcloud cli tool: "gcloud auth activate-service-account --key-file KEY_FILE"
// More infos under: https://cloud.google.com/vision/product-search/docs/auth
// - Create JWT and send it to GoogleCloud, requesting acces_token service_account
// - Use retrieved access_token as bearer to make requests to Vision API

pub mod auth;
pub mod todoist;
pub mod types;
pub mod utils;
pub mod vision_api;

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use todoist::Todoist;
//use vision_api::img_data_to_string;

/*
pub async fn todoist_from_handwriting(img_data: String) -> io::Result<()> {
    // println!("image-data from frontend: {}", img_data);
    let list_as_string = img_data_to_string(img_data.to_string()).await?;
    let digital_list = list_as_string.split_terminator('\n');
    let todoist = Todoist::default();
    match todoist.make_shopping_list(digital_list).await {
        Ok(res) => println!(
            "lig.rs/todoist_from_handwriting(): Successfully created a list on your todoist. Response was: {:#?}",
            res
        ),
        Err(e) => eprintln!("Could not create a list on your todoist. Err: {}", e),
    }
    Ok(())
}
*/
