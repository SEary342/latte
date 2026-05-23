use crate::{
    cli::CleanupArgs,
    db::maintenance::{cleanup_unused_projects, cleanup_unused_tags, cleanup_unused_tasks},
    errors::CliError,
};

pub fn handle(args: CleanupArgs) -> Result<(), CliError> {
    let CleanupArgs { dry_run } = args;
    let unused_tags = cleanup_unused_tags(dry_run)?;
    let unused_projects = cleanup_unused_projects(dry_run)?;
    let unused_tasks = cleanup_unused_tasks(dry_run)?;

    if dry_run {
        println!("Cleanup preview:");
    } else {
        println!("Cleanup complete:");
    }

    println!("  Tags removed: {}", unused_tags);
    println!("  Projects removed: {}", unused_projects);
    println!("  Tasks removed: {}", unused_tasks);

    Ok(())
}
