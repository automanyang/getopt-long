// -- main.rs --

use getopt_long::*;

// --

fn main() -> OptResult<()> {
    let longopts = &[
        Opt::new(None, Some('v'), HasArg::NoArgument, "show version.").unwrap(),
        Opt::new(None, Some('h'), HasArg::NoArgument, "help information.").unwrap(),
        Opt::new(None, Some('a'), HasArg::RequiredArgument, "add record to table.").unwrap(),
        Opt::new(None, Some('r'), HasArg::OptionalArgument, "remove record from table.").unwrap(),
        Opt::new(None, Some('m'), HasArg::NoArgument, "modify the record in table.").unwrap(),
        Opt::new(None, Some('q'), HasArg::NoArgument, "query the table.").unwrap(),
    ];

    usage("ex1", "this is ex1 example.", env!("CARGO_PKG_VERSION"), longopts);
    let p = getopt_long(longopts)?;
    println!("{}", p);
    Ok(())
}
