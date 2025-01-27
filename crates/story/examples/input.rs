use gpui::*;
use story::{Assets, InputStory};

pub struct Example {
    root: Entity<InputStory>,
}

impl Example {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let root = InputStory::view(window, cx);

        Self { root }
    }

    fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Example {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().p_4().size_full().child(self.root.clone())
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        story::init(cx);
        cx.activate(true);

        story::create_new_window("Input Example", Example::view, cx);
    });
}
