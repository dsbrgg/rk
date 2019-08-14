mod files;

use rk::Keeper;

fn main() {
    files::Files::test();
    let keeper = Keeper::new();
    keeper.append(); 
}
