use gtk::prelude::*;
use yaml_rust::{YamlLoader,Yaml};
use reqwest::Client;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

struct Window {
    window: gtk::ApplicationWindow
}

fn get_token() -> String {
    let mut conf = File::open("/home/wilwe/.config/lyr.yaml").expect("Failed to open config");
    let mut Str = String::new();
    conf.read_to_string(&mut Str).expect("file to read");
    let yaml = YamlLoader::load_from_str(&Str).unwrap();
    let yaml = &yaml[0];
    let token = yaml["token"].as_str().unwrap();
    token.to_string()
}

impl Window {
    fn new(app: gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::builder()
            .title("Discord2")
            .application(&app)
            .build();
        window.show();
        let but = gtk::Button::builder()
            .label("chuj")
            .build();
        window.set_child(Some(&but));
        but.connect_clicked(move |_| {
            println!("chuj");
        });
        Self {
            window
        }
    }
}
async fn send(message: String) -> Result<(), Box<dyn Error>>{
    let channel = "1292520084664746045";
    println!("channel = {}", channel);
    let url = format!("https://discord.com/api/v9/channels/{}/messages",channel);
    println!("url = {}", url);
    let params = [("content", message)];
    println!("params = {:?}", params);
    let client = Client::new();
    let res = client
        .post(url)
        .form(&params)
        .header("Authorization", get_token())
        .send()
        .await?;
    if res.status().is_success() {
        println!("success");
    } else {
        let error = res.text().await?;
        println!("Error: {}", error);
    }
    Ok(())
}

fn on_activate(app: &gtk::Application) {
    Window::new(app.clone());
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.wdiscord2")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
