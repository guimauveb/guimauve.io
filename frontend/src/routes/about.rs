use {crate::API_URL, yew::html, yew_functional::function_component};

#[function_component(About)]
pub fn about() -> Html {
    let stackoverflow_icon = "/media/images/about/c815fb0a-4f58-4420-b0ba-8612199449c8.webp";
    let mut stackoverflow_profile = String::with_capacity(API_URL.len() + stackoverflow_icon.len());
    stackoverflow_profile.push_str(API_URL);
    stackoverflow_profile.push_str(stackoverflow_icon);

    let linkedin_icon = "/media/images/about/84adeaba-827f-4854-aa03-f4429d8ebfeb.webp";
    let mut linkedin_profile = String::with_capacity(API_URL.len() + linkedin_icon.len());
    linkedin_profile.push_str(API_URL);
    linkedin_profile.push_str(linkedin_icon);

    html! {
        <div style="display: flex; justify-content: center; flex: 1;">
            <div style="flex: 1; max-width: 1024px;">
                <div style="align-items: center; display: flex; margin-bottom: 24px;">
                    <h1 class="heading">{"/about"}</h1>
                </div>
                <div flex="1">
                    <p style="white-space: break-spaces;">{"I am a passionate software developer who enjoys spending time in the various fields of programming. I am self-educated, and always keeping up to date and strengthening my skills play a big role in my everyday life.\n\nThe languages I have used the most include Rust, Typescript, Javascript, Python, C++, Bash script. I also tinker with Haskell from time to time. I have a strong interest in web development as well as OS development, embedded development, game development and compilers, even though most of my experience comes from web development."}</p>
                    <div style="margin-top: 24px; display: flex; justify-content: center;">
                        <p style="margin-right: 8px;">{"You can email me at "}</p>
                        <a class="email-container" href="mailto:contact@guimauve.io">{"contact@guimauve.io"}</a>
                    </div>
                    <div style="margin-top: 24px; margin-right: 8px; display: flex; justify-content: center;">
                        <a target="_blank" href="https://www.github.com/guimauveb/">
                            <div style="display: flex; font-size: 3em; margin-top: 8px; margin-right: 8px; align-items: center;">
                                <i class="fa fa-github"/>
                            </div>
                        </a>
                        <a href="https://stackoverflow.com/users/11676080/guimauve?tab=profile" target="_blank">
                            <img style="width: 4em;" src={stackoverflow_profile}/>
                        </a>
                        <a href="https://www.linkedin.com/in/guillaume-bournel-4a167b19b/" target="_blank">
                            <img style="width: 4em;" src={linkedin_profile}/>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
