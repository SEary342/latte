use crate::{
    cli::{ListArgs, SearchArgs},
    commands::search,
    db::metadata::{list_entities_by_column, list_named_entities},
    errors::CliError,
    ui::tables::render_metadata_table,
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
        let tasks = list_entities_by_column("tasks", "task_key")?;
        render_metadata_table("Tasks", &tasks);
    } else {
        let mut search_args = SearchArgs::default();

        if args.today {
            search_args.today = true;
        }

        search::handle(search_args)?;
    }

    Ok(())
}
