use {
    crate::{
        components::{
            box_component::BoxComponent,
            image::Image,
            text::{Text, TextVariant},
        },
        API_URL,
    },
    yew::html,
    yew_functional::function_component,
};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <BoxComponent display="flex" justify_content="center" flex="1">
            <BoxComponent flex="1" max_width="1024px">
                <BoxComponent align_items="center" display="flex" mb="24px">
                    <Text variant={TextVariant::Heading} value="/about" />
                </BoxComponent>
                <BoxComponent flex="1">
                    <Text value="I am a passionate software developer who enjoys spending time in the various fields of programming. I am self-educated, and always keeping up to date and strengthening my skills play a big role in my everyday life. I have found programming to be a lifestyle rather than a career choice.\n\nThe languages I have used the most include Rust, Typescript, Javascript, Python, C++, Bash script. I also tinker with Haskell from time to time. I have a strong interest in web development as well as OS development, embedded development, game development and compilers, even though most of my experience comes from web development."/>
                    <BoxComponent mt="24px" display="flex" justify_content="center">
                        <Text value="You can email me at "/>
                        <a class="email-container" href="mailto:contact@guimauve.io">{"contact@guimauve.io"}</a>
                    </BoxComponent>
                    <BoxComponent mt="24px" mr="8px" display="flex" justify_content="center">
                        <a target="_blank" href="https://www.github.com/guimauveb/">
                            <BoxComponent display="flex" font_size="3em" mt="8px" mr="8px" align_items="center">
                                <i class="fa fa-github"/>
                            </BoxComponent>
                        </a>
                        <a href="https://stackoverflow.com/users/11676080/guimauve?tab=profile" target="_blank">
                            <Image width="4em" src={API_URL.to_owned() + "/media/images/about/c815fb0a-4f58-4420-b0ba-8612199449c8.webp"}/>
                        </a>
                        <a href="https://www.linkedin.com/in/guillaume-b-4a167b19b/" target="_blank">
                            <Image width="4em" src={API_URL.to_owned() + "/media/images/about/84adeaba-827f-4854-aa03-f4429d8ebfeb.webp"}/>
                        </a>
                    </BoxComponent>
                </BoxComponent>
            </BoxComponent>
        </BoxComponent>
    }
}
