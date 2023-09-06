pub(crate) type AppResult<T> = anyhow::Result<T>;

pub(crate) fn myerror<T: AsRef<str>>(msg: T) -> anyhow::Error {
    anyhow::anyhow!("{message}", message = msg.as_ref())
}
