use leptos::prelude::*;
use rangular_aot::HostCell;
use rangular_host::{Host, HostError, Value};

use super::{
    SiteHost, interfire_project, itcy_project, opentrading_project, rangular_project,
    rustashop_project, serenade_project,
};

include!(concat!(env!("OUT_DIR"), "/rangular/project_page_view.rs"));

pub struct ProjectDefinition {
    pub name: &'static str,
    pub category: &'static str,
    pub headline: &'static str,
    pub description: &'static str,
    pub href: &'static str,
    pub link_label: &'static str,
    pub repository: &'static str,
    pub repository_label: &'static str,
    pub logo: &'static str,
    pub logo_alt: &'static str,
    pub accent: &'static str,
    pub wip: bool,
    pub features: &'static [(&'static str, &'static str)],
    pub architecture: &'static str,
    pub tech_stack: &'static [&'static str],
}

#[component]
pub fn ProjectPage(page: &'static str) -> impl IntoView {
    let project = project_definition(page).expect("project route has a definition");
    project_page_view(HostCell::new(ProjectPageHost(project)))
}

fn project_definition(page: &str) -> Option<&'static ProjectDefinition> {
    match page {
        "serenade" => Some(&serenade_project::PROJECT),
        "rustashop" => Some(&rustashop_project::PROJECT),
        "rangular" => Some(&rangular_project::PROJECT),
        "opentrading" => Some(&opentrading_project::PROJECT),
        "interfire" => Some(&interfire_project::PROJECT),
        "itcy" => Some(&itcy_project::PROJECT),
        _ => None,
    }
}

struct ProjectPageHost(&'static ProjectDefinition);

impl Host for ProjectPageHost {
    fn get(&self, name: &str) -> Option<Value> {
        let project = self.0;
        let value = match name {
            "name" => Value::from(project.name),
            "category" => Value::from(project.category),
            "headline" => Value::from(project.headline),
            "description" => Value::from(project.description),
            "href" => Value::from(project.href),
            "linkLabel" => Value::from(project.link_label),
            "repository" => Value::from(project.repository),
            "repositoryLabel" => Value::from(project.repository_label),
            "logo" => Value::from(project.logo),
            "logoAlt" => Value::from(project.logo_alt),
            "isSerenade" => Value::Bool(project.accent == "serenade"),
            "isRustashop" => Value::Bool(project.accent == "rustashop"),
            "isRangular" => Value::Bool(project.accent == "rangular"),
            "isInterfire" => Value::Bool(project.accent == "interfire"),
            "isOpenTrading" => Value::Bool(project.accent == "opentrading"),
            "isItcy" => Value::Bool(project.accent == "itcy"),
            "wip" => Value::Bool(project.wip),
            "features" => Value::List(
                project
                    .features
                    .iter()
                    .map(|(title, description)| Value::from(format!("{title}: {description}")))
                    .collect(),
            ),
            "architecture" => Value::from(project.architecture),
            "techStack" => Value::List(
                project
                    .tech_stack
                    .iter()
                    .map(|technology| Value::from(*technology))
                    .collect(),
            ),
            _ => return SiteHost::default().get(name),
        };
        Some(value)
    }

    fn call(&mut self, name: &str, _: &[Value]) -> Result<Value, HostError> {
        Err(HostError::UnknownMember(name.to_owned()))
    }
}
