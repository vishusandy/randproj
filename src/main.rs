extern crate rand;

/*
-n --num		num projs
	--number
-c --cat		category
	--category
		display all projs
-a --all
		display categories
if -a/-all and -c/--cat are present
		display projs in a category
*/
extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};
mod genproj;

fn main() {
	let mut numprojs = 0;
	let mut category = String::new();
	let mut showall = false;
	let mut showcats = false;
	
	{
		let mut ap = ArgumentParser::new();
		ap.set_description("Get random project ideas.");
		ap.refer(&mut numprojs).add_option(&["-n", "--num"], Store, "Number of project ideas to return");
		ap.refer(&mut category).add_option(&["-c", "--cat"], Store, "Category to choose from");
		ap.refer(&mut showall).add_option(&["-a", "--all"], StoreTrue, "Show all projects, or if category is specified show all projects within a category");
		ap.refer(&mut showcats).add_option(&["--cats", "--allcats", "--allcategories", "--categories"], StoreTrue, "Show all categories");
		ap.parse_args_or_exit();
	}
	
	/* if(numprojs != 0 && category != "" && showall != false)
		
	} else {
		
	} */
	genproj::gen(numprojs, category, showall, showcats);
}
