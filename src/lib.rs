#[macro_export]
macro_rules! volo {
  (
    $cls:ident
    $($name:ident($self:ident,$req:ident:$in:ident) -> $out:ident { $($body:tt)* })*
  ) => {
pub struct S;
use volo_grpc::{BoxError, Request, Response, Status};

#[volo::async_trait]
impl $cls for S {
$(
  async fn $name(&$self, $req: Request<$in>) -> Result<Response<$out>, Status> {
    let result: Result<_,BoxError> = async move {
        Ok({$($body)*})
    }.await;
    match result {
      Ok(r) => Ok(Response::new(r)),
      Err(err) => Err(err.into()),
    }
  }
)*
}
    }
}
