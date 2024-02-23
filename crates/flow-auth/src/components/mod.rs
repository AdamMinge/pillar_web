mod auth;
mod authenticated;
mod no_authenticated;

pub use auth::*;
pub use authenticated::*;
pub use no_authenticated::*;

use yew::prelude::*;

fn missing_context() -> Html {
    html!(<div> { "Unable to find Auth context! This element needs to be wrapped into an `Auth` component somewhere in the hierarchy"} </div>)
}
