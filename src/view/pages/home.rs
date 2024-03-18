use leptos::*;
use crate::view::components::display_card::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home">
            <div class="hero">
                <div class="background">
                    <div class="tagline">
                        <h1>"Bear Flinn"</h1>
                        <h2>"Knowledgeable, Articulate, Adaptable."</h2>
                    </div>
                </div>
                <img class="profile" src="Pfp.jpg" alt="Bear Flinn Headshot"/>
            </div>
            <div class="about">
                <h1>"About Me"</h1>
                <div class="content">
                    <p class="description">
                        "Chronically curious lifelong gamer with a passion for teaching others. What started with building computers with my Dad or Minecraft servers for my friends has become so much more. My time in IT and eSports has shown me the power of technology and gaming to bring people together. Now, I’m on a mission to demystify those industries and make them more accessible to the average person. As a technical writer, I have a practiced talent for making even the most dry and dense subjects engaging and accessible. If you need compelling technical writing or gaming content, I’m your guy. "
                    </p>
                    <DisplayCard
                        link = "/resume".to_string()
                        link_is_external = false
                        link_title = "Resume".to_string()
                    >
                        <h3 class="title">"Want to know more?"</h3>
                        <p> "My full background can be found on my resume page."</p>
                        <p class="click-me">"Click to Open"</p>
                    </DisplayCard>
                </div>
            </div>
            <div class="what-i-care-about">
                <h1>"What I Care About"</h1>
                <div class="content">
                    <div class="my-values">
                        <h2>"My Values"</h2>
                        <DisplayCard>
                            <div>
                                <h3>"Accessibility"</h3>
                                <p>"I believe everyone should have equal access to information, opportunities, and resources, regardless of their abilities. I strive to create inclusive experiences and advocate for accessibility in my work"</p>
                            </div>
                        </DisplayCard>
                        <DisplayCard>
                            <div>
                                <h3>"Communication"</h3>
                                <p>"I believe clear, honest communication is the foundation of success. I aim to listen actively, articulate ideas effectively, and collaborate towards shared goals."</p>
                            </div>
                        </DisplayCard>
                        <DisplayCard>
                            <div>
                                <h3>"Empathy"</h3>
                                <p>"I am a firm believer in empathy. I strive to understand and appreciate others' perspectives, and strive to be a part of their journey."</p>
                            </div>
                        </DisplayCard>
                    </div>
                    <img src="playing-dnd.jpg" alt="Bear Flinn Playing Dungeons and Dragons" class="dnd"/>
                </div>
            </div>
            <div class="what-i-do">
                <h1>"What I Do"</h1>
                <div class="content">
                    <div class="cards">
                        <DisplayCard
                                link = "/portfolio".to_string()
                                link_is_external = false
                                link_title = "Portfolio".to_string()
                            >
                                <h2 class="title">"Freelance Writer"</h2>
                                <p>"I'm a technical writer specializing in creating engaging, accessible content for the gaming and tech industries. With a unique blend of expertise in IT, eSports, and interpersonal communication, I help businesses break down complex topics into clear, compelling content."</p>
                                <p class="click-me">"Check out my Portfolio"</p>
                            </DisplayCard>
                            <DisplayCard
                                link = "https://grizzlysgamingden.com/".to_string()
                                link_is_external = true
                                link_title = "Grizzly's Gaming Den".to_string()
                            >
                                <h2 class="title">"Business Owner"</h2>
                                <p>"Grizzly’s Gaming Den is a passion project that aims to address America’s need for inclusive 3rd places. By building a community focused game store that puts people first I’m creating an inclusive and accessible place that anyone can come to forge new connections."</p>
                                <p class="click-me">"Click to Learn More"</p>
                            </DisplayCard>
                    </div>
                    <div class="background-bar"></div>
                </div>
            </div>

        </div>
    }
}