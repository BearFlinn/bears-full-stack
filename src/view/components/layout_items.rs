use leptos::{html::Div, *};
use leptos_use::on_click_outside;

// The navigation menu
#[component]
pub fn NavMenu() -> impl IntoView {
    view! {
        <div class="nav-menu">
            <a class="nav-item" title="Home" href="/">"Home"</a>
            <a class="nav-item" title="Portfolio" href="/portfolio">"Portfolio"</a>
            <a class="nav-item" title="Resume" href="/resume">"Resume"</a> 
            <a class="nav-item" title="Contact" href="/contact">"Contact"</a>
            <a class="nav-item" title="Playground" href="/playground">"Playground"</a>
        </div>
    }
}

// The sidebar
#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar-container">
            <div class="sidebar">
                <a class="title-nav" title="Home" href="/">
                    <h1>"Bear Flinn"</h1>
                </a>
                <NavMenu/>
            </div>
        </div>
    }
}

// Mobile menu
#[component]
pub fn MobileNav() -> impl IntoView {
    // State to keep track of whether the menu is open or closed
    let (is_nav_open, set_nav_open) = create_signal(false);

    // Function to toggle the menu open/closed
    let toggle_nav = move |_| set_nav_open.update(|open| *open = !*open);

    // Reference to the menu element
    let menu_ref = create_node_ref::<Div>();

    // Listener to close the menu when the user clicks outside of it
    let _ = on_click_outside(menu_ref, move |_| set_nav_open.set(false));

    view! {
        // Overlay to darken the background when the menu is open
        <div class="mobile-nav-overlay" 
            style={move || {if is_nav_open.get() {"opacity: 1"} else {"opacity: 0"}}}/> 
        // The menu itself
        <div class="mobile-nav">
            // Button to toggle the menu open/closed
            <button 
                class="nav-toggle"
                on:click=toggle_nav
                title="Toggle Navigation Menu"
                style={move || {if is_nav_open.get() {"display: none"} else {"display: flex"}}}>
                <i class="material-icons">"menu"</i>
            </button>
            // The actual menu
            <div node_ref=menu_ref
                class={move || {if is_nav_open.get() {"nav-open"} else {"nav-closed"}}}
                on:click=toggle_nav
            >
                <NavMenu/>
            </div>
        </div>
    }
}
    

// The social media buttons
#[component]
pub fn SocialButtons() -> impl IntoView {
    view! {
        <div class="social-buttons">
            <a class="sm-button" title="Bear Flinn's TikTok" href="https://www.tiktok.com/@bearflinn" target="_blank">
                <i class="fab fa-tiktok social_icon"></i>
            </a>
            <a class="sm-button" title="Bear Flinn's Twitter" href="https://twitter.com/TheBearFlinn" target="_blank">
                <i class="fab fa-twitter social_icon"></i>
            </a>
            <a class="sm-button" title="Bear Flinn's LinkedIn" href="https://www.linkedin.com/in/bear-flinn/" target="_blank">
                <i class="fab fa-linkedin social_icon"></i>
            </a>
            <a class="sm-button" title="Bear Flinn's Instagram" href="https://www.instagram.com/thebearflinn/" target="_blank">
                <i class="fab fa-instagram social_icon"></i>
            </a>
        </div>
    }
}

// The footer
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="footer">
            <div class="socials">
                <h2>"Check out my socials!"</h2>
                <SocialButtons/>
            </div>
            <p class="copyright">"Copyright © 2024 Bear Flinn. All rights reserved."</p>
        </div>
    }
}
