use yew::prelude::*;

use super::IconProps;

pub struct Twitter {
    class: Option<String>,
}

impl Component for Twitter {
    type Message = ();
    type Properties = IconProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { class: props.class }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg class=self.class.clone().unwrap_or(String::default()) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                <path d="m512 97.248c-19.04 8.352-39.328 13.888-60.48 16.576 21.76-12.992 38.368-33.408 46.176-58.016-20.288 12.096-42.688 20.64-66.56 25.408-19.264-20.512-46.72-33.216-76.672-33.216-58.112 0-104.9 47.168-104.9 104.99 0 8.32 0.704 16.32 2.432 23.936-87.264-4.256-164.48-46.08-216.35-109.79-9.056 15.712-14.368 33.696-14.368 53.056 0 36.352 18.72 68.576 46.624 87.232-16.864-0.32-33.408-5.216-47.424-12.928v1.152c0 51.008 36.384 93.376 84.096 103.14-8.544 2.336-17.856 3.456-27.52 3.456-6.72 0-13.504-0.384-19.872-1.792 13.6 41.568 52.192 72.128 98.08 73.12-35.712 27.936-81.056 44.768-130.14 44.768-8.608 0-16.864-0.384-25.12-1.44 46.496 29.984 101.6 47.104 161.02 47.104 193.15 0 298.75-160 298.75-298.69 0-4.64-0.16-9.12-0.384-13.568 20.832-14.784 38.336-33.248 52.608-54.496z" />
            </svg>
        }
    }
}
