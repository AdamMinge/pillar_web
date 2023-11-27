use yew::prelude::*;
use yew_nested_router::prelude::{use_router as use_nedsted_router, *};

#[hook]
pub fn use_router<T: Target + Clone>(target: T) -> Callback<()> {
    let router_ctx =
        use_nedsted_router::<T>().expect("Must be a child of a Router or Nested component");

    use_callback((), move |_, ()| {
        router_ctx.push(target.clone());
    })
}
