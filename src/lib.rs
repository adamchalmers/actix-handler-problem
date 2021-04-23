use actix::prelude::*;
use anyhow::Result;

#[derive(Clone)]
pub struct MyActor {
    pub input: Vec<u32>,
    pub output: Vec<u32>,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

/// When this is received, move a value from input to output,
/// and return it.
#[derive(Message)]
#[rtype(result = "Result<u32>")]
pub struct MyMsg {}

impl Handler<MyMsg> for MyActor {
    type Result = ResponseActFuture<Self, Result<u32>>;

    fn handle(&mut self, msg: MyMsg, ctx: &mut Context<Self>) -> Self::Result {
        let fut = async move { () }.into_actor(self).then(|_, actor, _ctx| {
            let f = async move {
                if let Some(val) = actor.input.pop() {
                    actor.output.push(val.clone());
                    Ok(val)
                } else {
                    Err(anyhow::anyhow!("No more input values!"))
                }
            }
            .into_actor(actor);
            Box::pin(f)
        });
        Box::pin(fut)
    }
}
