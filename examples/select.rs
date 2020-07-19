///
/// Rust Firebird Client
///
/// Example of select
///
/// You need create a database with this table:
/// create table test (col_a int generated by default as identity, col_b float, col_c varchar(10));
///
/// You can use the insert example to populate
/// the database ;)
///
use rsfbclient::Connection;

fn main() {
    let conn = Connection::open(
        "localhost".to_string(),
        3050,
        "examples.fdb".to_string(),
        "SYSDBA".to_string(),
        "masterkey".to_string(),
    )
    .expect("Error on connect");

    let tr = conn
        .start_transaction()
        .expect("Error on start the transaction");

    let stmt = tr
        .prepare("select col_a, col_b, col_c from test".to_string())
        .expect("Error on prepare the insert");

    let mut rows = stmt
        .query_simple()
        .expect("Error on execute the prepared insert");

    println!("| col_a | col_b | col_c   |");
    println!("| ----- | ----- | ------- |");
    loop {
        let row_op = rows.fetch().expect("Error on fetch the row");

        if let Some(row) = row_op {
            let col_a: i32 = row.get(0).expect("Error on get the value from 1° column");
            let col_b: f32 = row.get(1).expect("Error on get the value from 2° column");
            let col_c: String = row.get(2).expect("Error on get the value from 3° column");

            println!("| {:^5} | {:^5} | {:7} |", col_a, col_b, col_c);
        } else {
            break;
        }
    }

    tr.commit().expect("Error on commit the transaction");

    conn.close().expect("Error on close the connection");
}
