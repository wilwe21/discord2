use gtk::prelude::*;
use yaml_rust::{YamlLoader,Yaml};
use reqwest::blocking::{Client};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

use std::fs::File;
use std::io::prelude::*;

struct Window {
    window: gtk::ApplicationWindow
}

fn get_token() -> String {
    let mut conf = File::open("/home/wilwe/.config/lyr.yaml").expect("Failed to open config");
    let mut Str = String::new();
    conf.read_to_string(&mut Str).expect("file to read");
    let yaml = YamlLoader::load_from_str(&Str).unwrap();
    let yaml = yaml[0].clone();
    let token = yaml["token"].as_str().unwrap().to_string();
    token
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
        but.connect_clicked(move |but| {
            send(but.label().unwrap().to_string())
        });
        Self {
            window
        }
    }
}
fn send(message: String) {
    let channel = "1292520084664746045";
    let url = format!("https://discord.com/api/v9/channels/{}/messages",channel);
    /*let mut head = HeaderMap::new();
    let token = HeaderValue::from_bytes(get_token().as_bytes());
    let hd = HeaderName::from_static("Authorization");
    head.insert("Authorization", token.expect("chuj"));
    println!("{:?}", head);*/
    let client = Client::new();
    let l = '{';
    let r = '}';
    println!("idzie");
    let body = format!("{}\"content\": \"{}\"{}", l,message.clone(),r);
    let res = client.post(url)
        .body(body)
        .header("Authorization", get_token())
        .send();
    println!("OK, {:?}", res);
    if let Ok(res) = res {
        println!("OK, {:?}", res);
    } else {
        println!("err {:?}", res.err());
    }
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
