use shipyard::prelude::*;

struct NonSendStruct(core::marker::PhantomData<*const ()>);

unsafe impl Sync for NonSendStruct {}

#[system(NonSendSys)]
fn run(_: NonSend<&NonSendStruct>, _: NonSend<&mut NonSendStruct>) {}

#[system(NonSendSysUnique)]
fn run(_: NonSend<&NonSendStruct>, _: Unique<NonSend<&mut NonSendStruct>>) {}

#[system(UniqueNonSendSys)]
fn run(_: Unique<NonSend<&NonSendStruct>>, _: NonSend<&mut NonSendStruct>) {}

#[system(UniqueUnique)]
fn run(_: Unique<NonSend<&NonSendStruct>>, _: Unique<NonSend<&mut NonSendStruct>>) {}

fn main() {}
