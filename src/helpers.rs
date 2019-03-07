use failure::{Fallible, ResultExt};

pub trait ErrorHelp {
    type Ok;
    fn ctx(self, msg: &'static str) -> Fallible<Self::Ok>;
}

impl<T, E> ErrorHelp for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    type Ok = T;
    fn ctx(self, msg: &'static str) -> Fallible<T> {
        Ok(self.map_err(failure::Error::from).context(msg)?)
    }
}

pub fn likely(b: bool) -> bool {
    unsafe { std::intrinsics::likely(b) }
}

// pub fn unlikely(b: bool) -> bool {
//     unsafe { std::intrinsics::unlikely(b) }
// }
