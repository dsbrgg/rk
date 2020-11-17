use std::io;

use cli_table::{
    format::{CellFormat},
    Cell, 
    Table, 
    Row
};

pub fn find_table(password: String) -> io::Result<()> {
    let bold = CellFormat::builder().bold(true).build();
    let mut rows = vec![
        Row::new(vec![
            Cell::new(&format!("Password"), bold),
        ]),
        Row::new(vec![
            Cell::new(&password, Default::default())
        ])
    ];

    let table = Table::new(rows, Default::default()).unwrap();

    table.print_stdout()
}
