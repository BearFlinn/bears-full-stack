use leptos::*;

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
