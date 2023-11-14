mod error;
pub use error::*;

pub struct Ctx {
    user_id: u64,
}

impl Ctx {
    pub fn new(user_id: u64) -> CtxResult<Self> {
        if user_id == 0 {
            ctx_error::IllageUserIdError.fail()
        } else {
            Ok(Self { user_id })
        }
    }

    pub fn root_user() -> Self {
        Self { user_id: 0 }
    }
}
