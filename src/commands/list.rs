use crate::{
    cli::{ListArgs, SearchArgs},
    commands::search,
    db::metadata::{list_named_entities, list_tasks_with_descriptions},
    errors::CliError,
    ui::tables::{render_metadata_table, render_tasks_table},
};

pub fn handle(args: ListArgs) -> Result<(), CliError> {
    if args.tag {
        let tags = list_named_entities("tags")?;
        render_metadata_table("Tags", &tags);
    } else if args.project {
        let projects = list_named_entities("projects")?;
        render_metadata_table("Projects", &projects);
    } else if args.activity {
        let activities = list_named_entities("activity_types")?;
        render_metadata_table("Activity Types", &activities);
    } else if args.task {
        let tasks = list_tasks_with_descriptions()?;
        render_tasks_table(&tasks);
    } else {
        let mut search_args = SearchArgs::default();

        if args.today {
            println!("== Today's Logs ==");
            search_args.today = true;
        } else {
            println!("== All Logs ==");
        }

        search::handle(search_args)?;
    }

    Ok(())
}
