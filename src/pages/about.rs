use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="about-page container">
            <Title text="About - Tyler Harpool"/>

            <div class="about-header">
                <h1>"About Tyler Harpool"</h1>
                <div class="about-image">
                    <img src="/images/profile.png" alt="Tyler Harpool"/>
                </div>
            </div>

            <div class="about-content">
                <section class="bio">
                    <p>
                        "My journey in technology began early—all the way back in 5th grade when I wrote my first lines of code.
                        That childhood curiosity evolved into a lifelong passion for building and architecting technology that solves real problems."
                    </p>
                    <p>
                        "Today, I'm an Enterprise Solutions Architect, designing and implementing comprehensive technology strategies for organizations.
                        Prior to this role, I worked as a software engineer, developing applications and systems from the ground up.
                        This hands-on experience gives me a unique perspective when architecting enterprise-level solutions,
                        as I understand the technical challenges and considerations at every level of implementation."
                    </p>
                    <p>
                        "In my current role, I bridge the gap between business requirements and technological capabilities,
                        creating scalable, efficient architectures that meet both immediate needs and allow for future growth.
                        I particularly enjoy working with emerging technologies like Rust and exploring frameworks such as Leptos
                        to push the boundaries of what's possible in enterprise applications."
                    </p>
                </section>

                <section class="skills">
                    <h2>"Technical Expertise"</h2>
                    <div class="skills-grid">
                        <div class="skill-category">
                            <h3>"Architecture"</h3>
                            <ul>
                                <li>"System design"</li>
                                <li>"Microservices"</li>
                                <li>"Cloud architecture"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Languages"</h3>
                            <ul>
                                <li>"Rust"</li>
                                <li>"JavaScript/TypeScript"</li>
                                <li>"Python"</li>
                                <li>"Go"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Frontend"</h3>
                            <ul>
                                <li>"Leptos"</li>
                                <li>"React"</li>
                                <li>"Vue.js"</li>
                                <li>"HTML/CSS"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Backend"</h3>
                            <ul>
                                <li>"Axum"</li>
                                <li>"Node.js"</li>
                                <li>"Django"</li>
                                <li>"Express"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Databases"</h3>
                            <ul>
                                <li>"PostgreSQL"</li>
                                <li>"MongoDB"</li>
                                <li>"SQLite"</li>
                            </ul>
                        </div>
                        <div class="skill-category">
                            <h3>"Cloud & DevOps"</h3>
                            <ul>
                                <li>"AWS"</li>
                                <li>"Azure"</li>
                                <li>"Docker"</li>
                                <li>"Kubernetes"</li>
                                <li>"CI/CD pipelines"</li>
                            </ul>
                        </div>
                    </div>
                </section>

                <section class="closing">
                    <p>
                        "When I'm not designing solutions, I enjoy contributing to open-source projects,
                        writing technical articles, and mentoring aspiring developers. I believe in creating
                        technology that is not only functional but also accessible and user-friendly."
                    </p>

                    <p>
                        "Feel free to explore my projects or reach out if you'd like to discuss technology
                        strategy or collaborate on something exciting!"
                    </p>
                </section>

                <section class="contact">
                    <h2>"Get in Touch"</h2>
                    <div class="social-links">
                        <a href="https://github.com/tyler-harpool" target="_blank" rel="noopener noreferrer" class="social-link">
                            <span class="social-icon">Github</span>
                            "GitHub"
                        </a>
                        <a href="https://www.linkedin.com/in/tyler-harpool-16a487159/" target="_blank" rel="noopener noreferrer" class="social-link">
                            <span class="social-icon">Linkedin</span>
                            "LinkedIn"
                        </a>
                    </div>
                </section>
            </div>
        </div>
    }
}
