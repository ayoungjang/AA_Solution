use yew::prelude::*;

// A simple upload component
#[function_component(UploadFile)]
pub fn upload_file() -> Html {
    html! {
        <div>
            <input type="file" />
            <button>{"Upload"}</button>
        </div>
    }
}
