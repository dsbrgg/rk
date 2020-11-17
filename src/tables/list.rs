use std::io;

use cli_table::{
    format::{CellFormat},
    Cell, 
    Table, 
    Row
};

pub fn list_table(entities: Vec<String>) -> io::Result<()> {
    let bold = CellFormat::builder().bold(true).build();
    let mut rows = vec![
        Row::new(vec![
            Cell::new(&format!("Entity"), bold)
        ])
    ];

    for entity in entities {
        let row = Row::new(vec![Cell::new(&entity, Default::default())]);

        rows.push(row);
    }

    let table = Table::new(rows, Default::default()).unwrap();

    table.print_stdout()
}
