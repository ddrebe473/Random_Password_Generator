use yew::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[function_component(Password)]
fn password() -> Html {
    let password = use_state(|| String::new()); // Initialize state with an empty string

    let generate_password = {
        let password = password.clone(); // Clone state to use in callback
        Callback::from(move |_| {
            let mut characters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()+-?<>_0123456789"
                .chars()
                .collect();

            let mut rng = thread_rng();
            characters.shuffle(&mut rng);

            let new_password: String = characters.iter().take(16).collect(); // Generate a new password
            password.set(new_password); // Update state with the new password
        })
    };

    html! {
        <div>
            <h1>{ "Click button for a random password" }</h1>
            <button onclick={generate_password}>{ "Click me" }</button>
            <h2 id="demo">{ (*password).clone() }</h2> // Display current password
        </div>
    }
}

fn main() {
    yew::Renderer::<Password>::new().render(); // Corrected rendering method
}
