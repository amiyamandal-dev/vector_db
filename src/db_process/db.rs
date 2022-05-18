use rocksdb::{Options, DB};

fn create_db_to_write(db_path: String) -> Result<rocksdb::DB, rocksdb::Error> {
    let db = match DB::open_default(db_path) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    Ok(db)
}

fn create_db_to_read(
    db_path: String,
    error_if_log_file_exist: bool,
) -> Result<rocksdb::DB, rocksdb::Error> {
    let mut opts = Options::default();
    opts.increase_parallelism(4);
    let db = DB::open_for_read_only(&opts, db_path, error_if_log_file_exist)?;
    Ok(db)
}
