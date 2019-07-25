use rk::Locker;

fn main() {
    let lock = Locker::new();
    lock.write();
}
