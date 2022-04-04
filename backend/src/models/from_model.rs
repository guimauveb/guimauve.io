use crate::diesel::PgConnection;

pub trait FromModel<M> {
    fn from_model(m: M, connection: Option<&PgConnection>) -> Self;
}
