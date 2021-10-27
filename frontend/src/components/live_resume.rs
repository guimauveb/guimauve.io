#[cfg(feature = "editable")]
use {
    crate::{
        components::{
            box_component::BoxComponent, hr::Hr, resume_project::ResumeProject, text::Text,
        },
        entities::interfaces::{IProject, Status},
        service::{future::handle_future, projects::get_resume_projects},
    },
    std::collections::HashMap,
    yew::html,
    yew_functional::{function_component, use_effect_with_deps, use_state},
};

#[cfg(feature = "editable")]
#[function_component(LiveResume)]
pub fn resume() -> Html {
    let (resume_projects, set_resume_projects) = use_state(move || HashMap::default());

    use_effect_with_deps(
        move |_| {
            let future = async { get_resume_projects().await };
            handle_future(
                future,
                move |data: Result<HashMap<i32, IProject>, Status>| {
                    match data {
                        Ok(projects) => set_resume_projects(projects),
                        Err(_) => (),
                    };
                },
            );
            || {}
        },
        (),
    );
    html! {
        <BoxComponent display="flex" justify_content="center" flex="1">
            <BoxComponent flex="1" max_width="1024px" flex_direction="column">
                <BoxComponent display="flex" mb="20px" flex_direction="column">

                    // DETAILS
                    <BoxComponent mb="20px">
                        <BoxComponent mb="8px">
                            <Text value="Guillaume Bournel"/>
                        </BoxComponent>
                        <BoxComponent mb="8px">
                            <Hr/>
                        </BoxComponent>
                        <BoxComponent display="flex" flex="1">
                            <BoxComponent display="flex" flex_direction="column" flex="0 328px">
                                <BoxComponent display="flex" mb="8px">
                                    <i style="margin-right: 8px;" class="fa fa-stack-overflow"/>
                                    <Text value="/users/11676080/guimauve"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="8px">
                                    <i style="margin-right: 8px;" class="fa fa-linkedin"/>
                                    <Text value="/in/guillaume-b-4a167b19b" />
                                </BoxComponent>
                                <BoxComponent display="flex" mb="8px">
                                    <i style="margin-right: 8px;" class="fa fa-envelope-o"/>
                                    <Text value="contact@guimauve.io"/>
                                </BoxComponent>
                            </BoxComponent>
                            <BoxComponent display="flex" flex_direction="column" flex="0">
                                <BoxComponent display="flex" mb="8px">
                                    <i style="margin-right: 8px;" class="fa fa-github"/>
                                    <Text value="/guimauveb" />
                                </BoxComponent>
                                <Text value="guimauve.io" />
                            </BoxComponent>

                            <BoxComponent display="flex" flex="1" justify_content="flex-end">
                                <Text value="Nantes, FR"/>
                            </BoxComponent>
                        </BoxComponent>
                    </BoxComponent>

                    // ABOUT ME
                    <BoxComponent mb="20px">
                        <BoxComponent mb="8px">
                            <Text font_weight="bold" value="ABOUT ME"/>
                        </BoxComponent>
                        <BoxComponent mb="8px">
                            <Hr/>
                        </BoxComponent>
                        <Text value="I am a passionate software developer who enjoys spending time in the various fields of programming. I am self-educated, and always keeping up to date and strengthening my skills play a big role in my everyday life. I have found programming to be a lifestyle rather than a career choice.\nThe languages I have used the most include Rust, Typescript, Javascript, Python, C++, Bash script. I also tinker with Haskell from time to time. I have a strong interest in web development as well as OS development, embedded development, game development and compilers, even though most of my experience comes from web development. I am currently focusing on Rust and functional programming."/>
                    </BoxComponent>

                    // OBJECTIVE
                    <BoxComponent mb="20px">
                        <BoxComponent mb="8px">
                            <Text font_weight="bold" value="OBJECTIVE"/>
                        </BoxComponent>
                        <BoxComponent mb="8px">
                            <Hr/>
                        </BoxComponent>
                        <Text value="A position developing software and services."/>
                    </BoxComponent>

                    // SKILLS
                    <BoxComponent mb="20px">
                        <BoxComponent mb="8px">
                            <Text font_weight="bold" value="SKILLS"/>
                        </BoxComponent>
                        <BoxComponent mb="8px">
                            <Hr/>
                        </BoxComponent>

                        <BoxComponent display="flex" flex="1">
                            // Languages
                            <BoxComponent display="flex" flex="1" flex_direction="column">
                                <BoxComponent mb="8px">
                                    <Text value="Languages"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Rust"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Typescript/Javascript"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Python"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="C++"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="C"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Bash script"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Haskell"/>
                                </BoxComponent>
                            </BoxComponent>

                            // Libraries/Frameworks
                            <BoxComponent display="flex" flex="1" flex_direction="column">
                                <BoxComponent mb="8px">
                                    <Text value="Libraries and frameworks"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="React/React Native (mostly using Typescript)"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Redux"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Django"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Actix"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Yew"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="curses/ncurses"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="curl/curlpp"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Flask"/>
                                </BoxComponent>
                                <BoxComponent display="flex" mb="2px">
                                    <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="jQuery"/>
                                </BoxComponent>
                            </BoxComponent>

                            // DevOps/Services
                            <BoxComponent display="flex" flex="1" flex_direction="column" align_items="flex-end">
                                <BoxComponent>
                                    <BoxComponent mb="8px">
                                        <Text value="DevOps/Services"/>
                                    </BoxComponent>
                                    <BoxComponent display="flex" mb="2px">
                                        <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Git"/>
                                    </BoxComponent>
                                    <BoxComponent display="flex" mb="2px">
                                        <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Github"/>
                                    </BoxComponent>
                                    <BoxComponent display="flex" mb="2px">
                                        <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="AWS"/>
                                    </BoxComponent>
                                    <BoxComponent display="flex" mb="2px">
                                        <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Integromat"/>
                                    </BoxComponent>
                                    <BoxComponent display="flex" mb="2px">
                                        <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="nginx"/>
                                    </BoxComponent>
                                    <BoxComponent display="flex" mb="2px">
                                        <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Postgres"/>
                                    </BoxComponent>
                                    <BoxComponent display="flex" mb="2px">
                                        <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="MySQL"/>
                                    </BoxComponent>
                                    <BoxComponent display="flex" mb="2px">
                                        <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><Text value="Docker"/>
                                    </BoxComponent>
                                </BoxComponent>
                            </BoxComponent>
                        </BoxComponent>
                    </BoxComponent>

                    // SOFTWARE DEVELOPER EXPERIENCE
                    <BoxComponent mb="20px">
                        <BoxComponent mb="8px">
                            <Text font_weight="bold" value="SOFTWARE DEVELOPER EXPERIENCE"/>
                        </BoxComponent>
                        <BoxComponent mb="8px">
                            <Hr/>
                        </BoxComponent>

                        // Dashdoc
                        <BoxComponent mb="20px">
                            <BoxComponent display="flex" justify_content="space-between" mb="12px">
                                <Text font_weight="bold" value="Full stack developer - Dashdoc"/>
                                <Text font_weight="bold" value="January 2021 - Present"/>
                            </BoxComponent>
                            <Text font_size="0.8em" value="Working on every end of the codebase (back-end - integrations - front-end - mobile application) playing various developer roles. From implementing quick but much needed features (such as small API or UI changes needed for a new customer) to larger projects involving all ends of the product (such as bigger features that have been requested by a few clients for some time).\n\nSince I am the one to whom customer support goes first in case of trouble, I also play an important role in quick and critical bug resolution.\n\nI write a lot of React using Typescript, whether it be by taking part in the refactoring of the UI by creating 'ui-kit' generic and reusable components using functional components, or simply by adding new features. I also work on back-end features, such as adding and updating API endpoints and creating in-house tools to help customer support and operations team.\n\nI also work with Integromat to create new deployment scenarios, fix existing ones, add new modules and keep them up to date.\n\nThe languages and frameworks I have been using the most are Typescript/React and Python/Django."/>
                        </BoxComponent>

                        // guimauve
                        <BoxComponent mb="20px">
                            <BoxComponent display="flex" justify_content="space-between" mb="12px">
                                <Text font_weight="bold" value="Full stack developer - guimauve"/>
                                <Text font_weight="bold" value="August 2019 - Present"/>
                            </BoxComponent>
                            <Text font_size="0.8em" value="Creating all kinds of projects, consistently learning new technologies. Currently focusing on Rust and functional programming."/>
                        </BoxComponent>

                        // guimauve.io
                        <BoxComponent mb="20px">
                            <BoxComponent display="flex" justify_content="space-between" mb="12px">
                                <Text font_weight="bold" value="Blogger - guimauve.io"/>
                                <Text font_weight="bold" value="June 2020 - Present"/>
                            </BoxComponent>
                            <Text font_size="0.8em" value="I maintain an active blog discussing anything related to what I encounter during project development. I also write tutorials and guides."/>
                        </BoxComponent>

                        // Institut Sylvie
                        <BoxComponent mb="20px">
                            <BoxComponent display="flex" justify_content="space-between" mb="12px">
                                <Text font_weight="bold" value="Full stack web developer - institut-sylvie.fr"/>
                                <Text font_weight="bold" value="January 2020 - May 2020"/>
                            </BoxComponent>
                            <ResumeProject
                                project={match resume_projects.get(&1) {
                                    Some(institut_project) => institut_project.clone(),
                                    _ => IProject::default(),
                                }}
                            />
                        </BoxComponent>
                    </BoxComponent>

                    // OTHER EXPERIENCE
                    <BoxComponent mb="20px">
                        <BoxComponent mb="8px">
                            <Text font_weight="bold" value="OTHER EXPERIENCE"/>
                        </BoxComponent>
                        <BoxComponent mb="8px">
                            <Hr/>
                        </BoxComponent>

                        // Front Desk - Kyriad Hotel Nantes
                        <BoxComponent mb="20px">
                            <BoxComponent display="flex" justify_content="space-between" mb="12px">
                                <Text font_weight="bold" value="Front Desk - Kyriad Hotel - Nantes"/>
                                <Text font_weight="bold" value="December 2019 - July 2020"/>
                            </BoxComponent>
                            <Text font_size="0.8em" value="• Welcoming international guests in a warm and friendly manner.\n• Answering questions and addressing complaints\n• Knowing all essential aspects of the hotel operations.\n• Registering guests and managing the rooming chart.\n• Collecting payments by accepting cash, check, or charge payments from customers."/>
                            </BoxComponent>

                        // Auto Technician - Norauto Saint-Herblain
                        <BoxComponent mb="20px">
                            <BoxComponent display="flex" justify_content="space-between" mb="12px">
                                <Text font_weight="bold" value="Auto Technician - Norauto - Saint-Herblain"/>
                                <Text font_weight="bold" value="June 2018 - September 2019"/>
                            </BoxComponent>
                            <Text font_size="0.8em" value="Performed maintenance and repairs on all makes and models, including:\n\n• Oil changes\n• Tire rotations and replacement\n• Transmission flushes\n• Front-end alignments\n• Battery installations\n• Headlight/taillight installations\n\nConducted necessary repairs and took vehicles for test drives to verify soundness." />
                        </BoxComponent>
                    </BoxComponent>

                    // PROJECTS
                    <BoxComponent mb="20px">
                        <BoxComponent mb="8px">
                            <Text font_weight="bold" value="PROJECTS (See more projects at guimauve.io/projects)"/>
                        </BoxComponent>
                        <BoxComponent mb="8px">
                            <Hr/>
                        </BoxComponent>
                        <BoxComponent>
                            {for resume_projects.iter().filter(|(&id, _)| id != 1).  map(|(_, project)| {
                                    html! {
                                        <ResumeProject project={project} />
                                    }
                                })
                            }
                        </BoxComponent>
                    </BoxComponent>

                    // EDUCATION
                    <BoxComponent mb="20px">
                        <BoxComponent mb="8px">
                            <Text font_weight="bold" value="EDUCATION"/>
                        </BoxComponent>
                        <BoxComponent mb="8px">
                            <Hr/>
                        </BoxComponent>

                        // Law
                        <BoxComponent mb="20px">
                            <BoxComponent display="flex" justify_content="space-between" mb="12px">
                                <Text font_weight="bold" value="Bachelor of Law - University of Poitiers"/>
                            </BoxComponent>
                            <Text font_size="0.8em" value="Public law/Tax law"/>
                        </BoxComponent>

                        // MIP
                        <BoxComponent mb="20px">
                            <BoxComponent display="flex" justify_content="space-between" mb="12px">
                                <Text font_weight="bold" value="Mathematics - Physics - Computer Science - University of Nantes"/>
                            </BoxComponent>
                            <Text font_size="0.8em" value="Mathematics/Computer science"/>
                        </BoxComponent>

                    </BoxComponent>

                </BoxComponent>
            </BoxComponent>
        </BoxComponent>
    }
}
