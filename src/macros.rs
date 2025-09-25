macro_rules! try_with_log {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Something went wrong: {:?}", e);
                return Err(e.into());
            }
        }
    };
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("{}: {:?}",$msg, e);
                return Err(e.into());
            }
        }
    }
}

#[allow(unused)]
pub(crate) use try_with_log;