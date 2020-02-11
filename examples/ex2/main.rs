// -- main.rs --

use getopt_long::*;

// --

fn main() -> OptResult<()> {
    let longopts = &[
        Opt::new(None, Some('v'), HasArg::NoArgument, "show version.").unwrap(),
        Opt::new(None, Some('h'), HasArg::NoArgument, "help information.").unwrap(),
        Opt::new(Some("add".to_owned()), Some('a'), HasArg::RequiredArgument, "add record to table.").unwrap(),
        Opt::new(Some("remove".to_owned()), Some('r'), HasArg::OptionalArgument, "remove record from table.").unwrap(),
        Opt::new(Some("modify".to_owned()), Some('m'), HasArg::NoArgument, "modify the record in table.").unwrap(),
        Opt::new(Some("query".to_owned()), None, HasArg::NoArgument, "query the table.").unwrap(),
    ];

    usage("ex2", "this is ex2 example.", "1.0.1", longopts);
    let p = getopt_long(longopts)?;
    println!("Arguments:\n{}", p);
    Ok(())
}
