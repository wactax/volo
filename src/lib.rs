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
     match {
       async move {
         $($body)*
       }.await
     } {
       Ok(r) => Ok(r),
       Err(err) => Err(err.into()),
     }
  }
)*
}
    }
}
