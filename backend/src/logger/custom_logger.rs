use {
    crate::Pool,
    env_logger::filter::{Builder, Filter},
    log::{Log, Metadata, Record, SetLoggerError},
};

#[cfg(not(debug_assertions))]
use {
    crate::{diesel::prelude::*, models::logs::NewLog, schema::logs},
    std::thread,
};

const FILTER_ENV: &str = "LOG_LEVEL";

#[allow(dead_code)]
pub struct Logger {
    inner: Filter,
    pool: Pool,
}

impl Logger {
    pub fn new(pool: Pool) -> Logger {
        let mut builder = Builder::from_env(FILTER_ENV);

        Logger {
            inner: builder.build(),
            pool,
        }
    }

    pub fn init(pool: Pool) -> Result<(), SetLoggerError> {
        let logger = Self::new(pool);

        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    // Print logs to stdout in dev mode, store them in database in release.
    fn log(&self, record: &Record) {
        let (record_level, record) = (record.level().as_str(), record.args().to_string());

        #[cfg(debug_assertions)]
        println!("{} {}", &record_level, &record);

        #[cfg(not(debug_assertions))]
        {
            let pool = self.pool.clone();
            // Wanted to try using asynchronous code, but it seems way too complicated for the purpose of this blog.
            thread::spawn(move || {
                let conn = pool.get().unwrap();
                diesel::insert_into(logs::table)
                    .values(&NewLog {
                        record_level,
                        record: &record,
                    })
                    .execute(&conn)
                    .expect("Could not insert log");
            });
        }
    }

    fn flush(&self) {}
}
