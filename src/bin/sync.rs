fn wait_for(duration: std::time::Duration) {
    let handle = std::thread::spawn(move || std::thread::sleep(duration));
    handle.join().unwrap();
}

struct Song {
    name: String,
    duration: std::time::Duration,
}

fn learn_song() -> Song {
    wait_for(std::time::Duration::from_millis(30));
    Song { name: "Yellow Submarine".into(), duration: std::time::Duration::from_millis(50) }
}

fn sing(song: Song) {
    println!("I'm singing the song \"{}\"!", song.name);
    wait_for(song.duration);
    println!("I finished singing!");
}

fn dance() {
    println!("I'm dancing!");
    wait_for(std::time::Duration::from_millis(30));
    println!("I finished dancing!");
}

fn main() {
    let instant = std::time::Instant::now();
    let song = learn_song();
    sing(song);
    dance();
    println!("Program took: {}ms", instant.elapsed().as_millis());
}
