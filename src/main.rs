#![windows_subsystem = "windows"]
#[macro_use]
extern crate sciter;

use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use std::thread;

static mut VOLUME: f32 = 100.0;
static mut PAUSED: bool = false;
static mut CLICK: bool = false;
static mut MINIMIZED: bool = false;

use sciter::Value;
struct EventHandler;
impl EventHandler {
    fn set_volume(&self, volume: i32) -> () {
        unsafe { VOLUME = volume as f32 };
    }
    fn pause(&self, paused: bool) -> () {
        unsafe { PAUSED = paused };
    }
    fn click(&self) -> () {
        unsafe { CLICK = true };
    }
    fn start_at_login(&self, add: bool) -> sciter::Value {
        // https://users.rust-lang.org/t/how-to-make-my-exe-autorun-in-windows/49045/12
        use std::path::Path;
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
        let path = Path::new("Software")
            .join("Microsoft")
            .join("Windows")
            .join("CurrentVersion")
            .join("Run");
        let (key, disp) = hkcu.create_subkey(&path).unwrap();
        dbg!(&disp);
        let path = format!(
            "\"{}\"",
            std::env::current_exe()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        );
        if add {
            key.set_value("Rackety", &path).unwrap();
        } else {
            key.delete_value("Rackety").unwrap();
        }
        sciter::Value::from(true)
    }
}

impl sciter::EventHandler for EventHandler {
    fn document_complete(&mut self, root: sciter::HELEMENT, source: sciter::HELEMENT) {
        let root = sciter::Element::from(root);
        //let h1 = root.find_first("h1").unwrap().unwrap();
        std::thread::spawn(move || {
            monitor_keyboard();
        });

        use std::path::Path;
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
        let path = Path::new("Software")
            .join("Microsoft")
            .join("Windows")
            .join("CurrentVersion")
            .join("Run");
        let autostart = hkcu.open_subkey(&path).unwrap();
        let key: String = autostart
            .get_value("Rackety")
            .unwrap_or_else(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("Key doesn't exist");
                    String::from("")
                }
                std::io::ErrorKind::PermissionDenied => panic!("Access denied"),
                _ => panic!("{:?}", e),
            });

        if key != "" {
            root.eval_script(
                "
                document.querySelector('#start-at-login').checked = true;
                document.querySelector('#menu-start-at-login > button').checked = true;
            ",
            );
        }

        if unsafe { MINIMIZED } {
            root.eval_script(
                "
                document.querySelector('[role=window-minimize]').click();
                document.querySelector('#start-at-login').checked = true;
                document.querySelector('#menu-start-at-login > button').checked = true;
            ",
            );
        }
    }
    fn get_subscription(&mut self) -> Option<sciter::dom::event::EVENT_GROUPS> {
        Some(
            sciter::dom::event::default_events()
                | sciter::dom::event::EVENT_GROUPS::HANDLE_METHOD_CALL,
        )
    }
    dispatch_script_call!(
        fn set_volume(i32);
        fn pause(bool);
        fn click();
        fn start_at_login(bool);
    );
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        if (args[1] == "--minimized") {
            unsafe { MINIMIZED = true };
        }
    }

    sciter::set_options(sciter::RuntimeOptions::DebugMode(false)).unwrap();
    let archived = include_bytes!("../target/assets.rc");
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_EVAL as u8,
    ))
    .unwrap();
    let mut frame = sciter::Window::new();
    frame.event_handler(EventHandler {});
    frame.archive_handler(archived).unwrap();
    frame.load_file("this://app/html/main.html");
    frame.run_app();
}

fn monitor_keyboard(/*mut element: sciter::Element*/) {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];

    use soloud::*;
    let mut sl = Soloud::default().unwrap();

    let mut wav0 = audio::Wav::default();
    wav0.set_volume(unsafe { VOLUME });
    wav0.load_mem(include_bytes!("../sciter/wav/typewriter/0.wav"))
        .unwrap();

    let mut wav1 = audio::Wav::default();
    wav1.set_volume(unsafe { VOLUME });
    wav1.load_mem(include_bytes!("../sciter/wav/typewriter/1.wav"))
        .unwrap();

    let mut wav2 = audio::Wav::default();
    wav2.set_volume(unsafe { VOLUME });
    wav2.load_mem(include_bytes!("../sciter/wav/typewriter/2.wav"))
        .unwrap();

    let mut wav3 = audio::Wav::default();
    wav3.set_volume(unsafe { VOLUME });
    wav3.load_mem(include_bytes!("../sciter/wav/typewriter/3.wav"))
        .unwrap();

    let mut wav4 = audio::Wav::default();
    wav4.set_volume(unsafe { VOLUME });
    wav4.load_mem(include_bytes!("../sciter/wav/typewriter/4.wav"))
        .unwrap();

    let mut wav5 = audio::Wav::default();
    wav5.set_volume(unsafe { VOLUME });
    wav5.load_mem(include_bytes!("../sciter/wav/typewriter/5.wav"))
        .unwrap();

    let wavs = vec![&wav0, /*&wav1,*/ &wav2, &wav3, &wav4, &wav5];

    loop {
        let mouse: MouseState = device_state.get_mouse();
        //println!("{:?}", mouse.coords);
        //element.set_text(&format!("{:?}", mouse.coords).to_owned());

        let keys: Vec<Keycode> = device_state.get_keys();
        if (keys != prev_keys && !keys.is_empty()) || unsafe { CLICK } == true {
            println!("{:?}", keys);

            if (unsafe { PAUSED } == false || unsafe { CLICK } == true) {
                use rand::Rng;
                let num = rand::thread_rng().gen_range(0..5);

                sl.set_global_volume(unsafe { VOLUME } / 100.0);

                sl.stop_all();
                sl.play(wavs[num]);

                println!("{}", num);

                unsafe { CLICK = false };
            }
        }
        prev_keys = keys;
    }
}
