use std::io;

use cli_table::{
    format::{CellFormat},
    Cell, 
    Table, 
    Row
};

pub fn list_table(list: Vec<String>, accounts: bool) -> io::Result<()> {
    let bold = CellFormat::builder().bold(true).build();
    let header = if !accounts { format!("Entity") } else { format!("Account") };
    let mut rows = vec![
        Row::new(vec![
            Cell::new(&header, bold)
        ])
    ];

    for register in list {
        let row = Row::new(vec![Cell::new(&register, Default::default())]);

        rows.push(row);
    }

    let table = Table::new(rows, Default::default()).unwrap();

    table.print_stdout()
}
