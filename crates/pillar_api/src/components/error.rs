use yew::prelude::*;

pub(crate) fn missing_context() -> Html {
    html!(<div> { "Unable to find Auth context! This element needs to be wrapped into an `Auth` component somewhere in the hierarchy"} </div>)
}
