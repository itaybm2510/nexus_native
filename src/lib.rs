use android_activity::{AndroidApp, MainEvent, PollResult};

#[no_mangle]
fn android_main(app: AndroidApp) {
    println!("🛸 NEXUS NATIVE: המנוע העצמאי התחיל לרוץ!");

    loop {
        // האפליקציה מחכה לאירועים מהמערכת
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            match event {
                PollResult::Main(MainEvent::Start) => {
                    println!("🛸 NEXUS NATIVE: האפליקציה עלתה למסך!");
                }
                PollResult::Main(MainEvent::SaveState) => {
                    println!("🛸 NEXUS NATIVE: שמירת מצב אפליקציה.");
                }
                _ => {}
            }
        });
    }
}

