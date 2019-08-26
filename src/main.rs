mod files;

use rk::Keeper;
use files::LockerFiles;

fn main() {
    LockerFiles::new();
    let keeper = Keeper::new();
    keeper.append(); 
}
