//! Wait for laundry card to be presented to the reader,
//! then print card info

use freefare::Freefare;
use nfc::Context;
use nfc::version;

static MAX_DEVICES: usize = 16;

fn main() -> freefare::Result<()> {
    let context = Context::new().expect("Could not create context");

    println!("Using libnfc {}", version());
    println!("Using libfreefare <unknown version>");
    println!();

    // List devices, use first found
    let devices = context
        .list_devices(MAX_DEVICES)
        .expect("Could not get devices");
    if devices.len() == 0 {
        println!("No devices found");
        return Ok(());
    }
    println!("Found devices:");
    for device in &devices {
        println!(" - {device}");
    }
    let connstring = devices.get(0).expect("Could not get device from list");
    println!("Using: {connstring}");
    let device: nfc::Device = context.open(Some(connstring))?;
    println!();

    // List tags, use first found
    let tags = Freefare::get_tags(&device).expect("Could not get tags");
    if tags.len() == 0 {
        println!("No tags found");
        return Ok(());
    }
    println!("Found tags:");
    for tag in &tags {
        println!(
            " - {} - {:?}",
            tag.friendly_name().unwrap_or(String::from("<UNKNOWN>")),
            tag.tag_type()
        );
    }
    let tag = tags.get(0).expect("Could not get tag from list");
    println!(
        "Using: {} - {:?}",
        tag.friendly_name().unwrap_or(String::from("<UNKNOWN>")),
        tag.tag_type()
    );

    Ok(())
}
