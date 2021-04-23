use actix::prelude::*;

#[derive(Clone)]
pub struct MyActor {
    pub input: Vec<u32>,
    pub output: Vec<u32>,
}

impl MyActor {
    async fn shuffle(&mut self) -> Option<u32> {
        let val = self.input.pop();
        if let Some(val) = &val {
            self.output.push(*val);
        }
        val
    }
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

/// When this is received, move a value from input to output,
/// and return it.
#[derive(Message)]
#[rtype(result = "Option<u32>")]
pub struct MyMsg {}

impl Handler<MyMsg> for MyActor {
    type Result = ResponseActFuture<Self, Option<u32>>;

    fn handle(&mut self, msg: MyMsg, ctx: &mut Context<Self>) -> Self::Result {
        let fut = async move { () }.into_actor(self).then(|_, actor, _ctx| {
            let f = async move { actor.shuffle().await }.into_actor(actor);
            Box::pin(f)
        });
        Box::pin(fut)
    }
}
