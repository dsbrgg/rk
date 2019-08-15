mod files;

use rk::Keeper;
use files::LockerFiles;

fn main() {
    LockerFiles::test();
    let keeper = Keeper::new();
    keeper.append(); 
}
