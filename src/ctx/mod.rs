mod error;
pub use error::*;

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    pub fn new(user_id: i64) -> CtxResult<Self> {
        if user_id == 0 {
            ctx_error::IllageUserIdError.fail()
        } else {
            Ok(Self { user_id })
        }
    }

    pub fn root_ctx() -> Self {
        Self { user_id: 0 }
    }
}

impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
