use leptos::*;
use std::time::Duration;

/// "Typewriter text" component, will create the effect that some text is being typed out.
///
/// Note that to style the individual parts of this component, supply css for
/// `span.typewriter-text` to style to text, `span.input-cursor` to style the cursor
/// (if you want it animated, you should do that here), and `div.typing-container` to
/// style how these two elements are arranged (this should probably be a flex box with
/// them horizontal)
///
/// ## Arguments
/// - `text` - the text to type out
/// - `delay` - the delay between each character
/// - `class_name` - the class name for the parent component
#[component]
pub fn TypewriterText(
    cx: Scope,
    text: &'static str,
    delay: Duration,
    class_name: &'static str,
) -> impl IntoView {
    let (value, set_value) = create_signal(cx, ""); // set empty signal to update text

    /// function for updating the text value every `delay` timestamp
    fn update_text(i: usize, value_setter: WriteSignal<&str>, text: &'static str, delay: Duration) {
        // run only if haven't added all text yet
        if i < text.len() + 1 {
            value_setter(&text[..i]);

            // set timeout until next recursive call
            set_timeout(move || update_text(i + 1, value_setter, text, delay), delay);
        }
    }

    // start recursion
    update_text(1, set_value, text, delay);

    view! {
        cx,
        class = {class_name},
        <div class="typing-container">
            <span class="typewriter-text"> {value} </span>
            <span class="input-cursor"></span>
        </div>
    }
}
