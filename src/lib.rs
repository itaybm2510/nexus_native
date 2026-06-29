use android_activity::{AndroidApp, InputStatus, MainEvent, PollResult};

#[no_mangle]
fn android_main(app: AndroidApp) {
    // לוג ראשוני שמראה שהאפליקציה העצמאית עלתה בטלפון!
    println!("🛸 NEXUS NATIVE: המנוע העצמאי התחיל לרוץ על המכשיר!");

    loop {
        // המנוע מחכה לאירועים מהמסך של האנדרואיד (כמו נגיעה או סיבוב מסך)
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            match event {
                PollResult::Main(MainEvent::InitWindow) => {
                    println!("🛸 NEXUS NATIVE: המסך הגרפי נוצר בהצלחה!");
                    // כאן בעתיד נצייר את הכפתורים והעיצוב
                }
                PollResult::Main(MainEvent::DestroyWindow) => {
                    println!("🛸 NEXUS NATIVE: האפליקציה נסגרת.");
                }
                _ => {}
            }
        });
    }
}

