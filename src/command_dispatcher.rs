use prost::Message;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use chrono::Utc;

mod video {
    include!(concat!(env!("OUT_DIR"), "/video.rs"));
}

use video::Camera;

fn serialize_video_command(video: &Camera) -> Vec<u8> {
    let mut buf = Vec::new();
    video.encode(&mut buf).unwrap();
    buf
}

fn calculate_checksum(video: &Camera) -> u32 {
    let mut modified_video = video.clone();
    modified_video.header.as_mut().unwrap().id = 0;
    modified_video.header.as_mut().unwrap().timestamp = 0;

    let serialized_video = serialize_video_command(&modified_video);

    serialized_video.iter().map(|&byte| byte as u32).sum()
}

fn create_video_command() -> Camera {
    let mut video = Camera {
        header: Some(video::Header {
            id: 1,
            timestamp: Utc::now().timestamp() as u32,
        }),
        name: "John Doe".to_string(),
        contact_info: Some(video::video::ContactInfo::Email("john.doe@example.com".to_string())),
        // contact_info: Some(video::video::ContactInfo::Phone("123456".to_string())),
        address: "123 Main St, Anytown, USA".to_string(),
        footer: Some(video::Footer {
            checksum: 0,
        }),
    };
    video.footer.as_mut().unwrap().checksum = calculate_checksum(&video);
    video
}

fn write_serialized_video_to_file(data: &Vec<u8>, filename: &str) {
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();
}

fn read_video_from_file(filename: &str) -> Camera {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    Camera::decode(&*buf).unwrap()
}

fn main() {
    // Create a video
    let video_command = create_video_command();

    // Serialize video
    let serialized_video = serialize_video_command(&video_command);

    // Write serialized video to file
    write_serialized_video_to_file(&serialized_video, "video.dat");

    println!("Camera data serialized and written to file.");

    // Read and deserialize the video
    let deserialized_video = read_video_from_file("video.dat");

    // Display the video's information
    println!("---Header---");
    println!("Camera ID: {}", deserialized_video.header.unwrap().id);
    println!("Timestamp: {}", deserialized_video.header.unwrap().timestamp);
    println!("---Payload---");
    println!("Name: {}", deserialized_video.name);
    match &deserialized_video.contact_info {
        Some(video::video::ContactInfo::Email(email)) => {
            println!("Email: {}", email);
        }
        Some(video::video::ContactInfo::Phone(phone)) => {
            println!("Phone: {}", phone);
        }
        None => {
            println!("No contact info provided.");
        }
    }
    println!("Address: {}", deserialized_video.address);
    println!("---Footer---");
    println!("Checksum: {}", deserialized_video.footer.unwrap().checksum);
}