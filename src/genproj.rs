// extern crate rand;
use rand::{thread_rng, Rng};
/*
let mut rng = rand::thread_rng();
rng.shuffle(&mut secret_num);
*/

pub fn gen(mut pnum: usize, pcat: String, pall: bool, call: bool) {
	let mut rg = thread_rng();
	let cats = ["math", "text", "datastructures", "network", "objects", "concurrency", "files", "database", "graphics", "games", "algorithms", "cs", "security"];
	let mut projs = vec![
		vec!["Find Pi to the Nth digit", "Find e to the Nth digit", "Fibonacci sequence", "Generate prime numbers until user press stop", "Interest calculator", "Change in coins returned from random amount", "Binary <-> decimal <-> hex number converter", "unit converter", "factorial finder", "Ackermann function"],
		vec!["Short description (shorten text to x characters, round to nearest word)", "Pig Latin", "Count vowels", "Reverse a string", "Check if a string is a palindrome", "Count the number of words in a string", "RSS feed generator", "Favorite quotes log", "journal entry editor/creator"],
		vec!["Linked list", "Unrolled list", "Xor doubly linked list", "Stack", "Queue", "Binary Tree", "Hash table", "Binary Search Tree", "A*Star pathfinding algorithm", "Heap", "Hashstring", "Array", "Matrix", "Circular linked list", "Doubly linked list", "Depth first search", "Breadth first search", "Trie", "Suffix array/Suffix tree", "AVL Tree", "Spaly tree", "B Tree", "Segment Tree", "Red-black tree", "K dimensional tree", "Ternary search tree", "Inverted index"],
		vec!["(Secure) Ftp program", "Get internet time", "Chat application", "Fetch weather", "Download file over HTTP", "Download file over local network", "P2P File Transfer", "Mass emailer - manage groups of emailees, create and preview message to send"],
		vec!["Inventory manager", "Movie rental store - search inventory, check out & return movies (limited quantity of movies), featured movies, new movies, on-sale movies, return reminders", "Airline reservation system", "Hotel reservation system - manage hotels: types of rooms, # of rooms of each type, price of room types, number of rooms taken.  Manage reservations: user search for rooms given search parameters.", "Student grade book", "Bank account manager", "Library catalog", "Family tree generator", "Recipe manager"],
		vec!["Multi-threaded download manager", "Multi-threaded compression", "Multi-threaded encryption", "Multi-threaded file hashing", "Multi-threaded bulk thumbnailer", "Multi-threaded data structure search", "Multi-threaded Pi"],
		vec!["File explorer", "File sorter", "Bulk file renamer", "Compressed archive manager/creator", "Duplicate file detector"],
		vec!["TV Show tracker", "Movie rental store - search inventory, check out & return movies (limited quantity of movies)", "Airline reservation system", "Hotel reservation system - manage hotels: types of rooms, # of rooms of each type, price of room types, number of rooms taken.  Manage reservations: user search for rooms given search parameters.", "Travel planner: schedule/itinerary, accomodation info, budget, travel journal", "Forum", "Database backup tool", "Student Grade book", "Recipe manager", "Library catalog"],
		vec!["Slide show", "Brainstorming mind map", "Screen capture", "Image browser", "Image adjust: grayscale, contrast, brightness", "Screen saver", "Watermark application", "Bloom filter", "Image anti-aliasing", "Point in polygon - user creates polygon, determine whether clicks are inside or outside of polygon"],
		vec!["Battleship", "RPG", "Chess", "Checkers", "Hangman", "Crossword puzzle", "Frogger", "Pac-man", "Maze", "Tic-tac-toe", "Quiz creator & taker"],
		vec!["Greatest common denominator", "Least common multiple", "Euler totient", "Sieve of eratosthenes", "Ackermann function", "Midpoint circle algorithm", "Mandlebrot", "Mersenne twister"],
		vec!["Knapsack", "Travelling salesman", "Cigareete smokers problem", "Producer-consumer problem", "Readers writers problem", "Sleeping barber problem", "Man or boy problem", "Dining philosophers", "Turing machine", "Hanoi tower", "N-queens", "Shannon entropy calculator"],
		vec!["Secure transfer protocol (using either two or three parties)", "Secure hash implementation", "Modern encryption algorithm", "Matrix encryption algorithm", "Character frequency analysis"]
	];
	
    // rg.shuffle(&mut );
	// let mut rng = rand::thread_rng();
	// rng.shuffle(&mut secret_num);
	
	// let test = vec!["one", "two", "three"];
    // let index = cats.iter().position(|&r| r == "two").unwrap();
	if  call {
		println!("Showing all categories.");
		for i in 0..13 {
			println!("{}", cats[i]);
		}
	} else if pall {
		if pnum != 0 { //show projects from all categories
			println!("Showing {} random projects from all categories.", pnum);
			for i in 0..13 {
				let mut cnum = pnum;
				println!("Category: {}", cats[i]);
				rg.shuffle(&mut projs[i]);
				if pnum > projs[i].len() {
					cnum = projs[i].len();
				}
				for j in 0..cnum {
					println!("\t{}: {}", cats[i], projs[i][j]);
				}
			}
		} else {
			if pcat == "" { //show all projects in all categories
				println!("Showing all projects in all categories");
				for c in 0..13 {
					println!("Category: {}", cats[c]);
					for p in 0..projs[c].len() {
						println!("\t{}: {}", cats[c], projs[c][p]);
					}
				}
			} else { //show projects in a specific category
				let idx = cats.iter().position(|&r| r == pcat).unwrap_or(rg.gen_range(0, 12));
				if cats[idx] != pcat {
					println!("Category `{}` was not found, using random category `{}` instead", pcat, cats[idx]);
				}
				println!("showing all projects in the {} category", cats[idx]);
				for i in 0..projs[idx].len() {
					println!("\t{}: {}", cats[i], projs[idx][i]);
				}
			}
		}
	} else {
		if pnum == 0 {
			pnum = 1;
		}
		if pcat == "" { //generate projects from a random category
			println!("Showing {} random projects from a random category", pnum);
			if pnum == 1 {
				// let gcat = rg.choose(&cats);
				// println!("Project from the {} category.", gcat.unwrap());
				let gcati = rg.gen_range(0, 12);
				let gproj = rg.gen_range(0, cats[gcati].len()-1);
				// println!("Project from the {} category.", cats[gcati]);
				println!("{}: {}", cats[gcati], projs[gcati][gproj]);
				// let gproj = rg.choose(&projs[gcat]);
				// println!("Project: {} from the {} category.", cats[gcat], projs[gcat][gproj]);
			} else {
				// let mut catprojs = projs[];
				// rg.shuffle(&mut );
				for _ in 0..pnum {
					let gcati = rg.gen_range(0, 12);
					let gproj = rg.gen_range(0, projs[gcati].len()-1);
					println!("{}: {}", cats[gcati], projs[gcati][gproj]);
				}
			}
		} else { //generate projects from a specified category
			println!("Showing {} random projects from the `{}` category ", pnum, pcat);
				// let gcati = cats.iter().position(|&r| r == pcat).unwrap();
				let gcati = cats.iter().position(|&r| r == pcat).unwrap_or(rg.gen_range(0, 12));
				if cats[gcati] != pcat {
					println!("Category {} not found.  Using random category {} instead.", pcat, cats[gcati]);
				}
				// let gcati = rg.gen_range(0, 12);
				rg.shuffle(&mut projs[gcati]);
				if pnum > projs[gcati].len() {
					pnum = projs[gcati].len();
				}
				for i in 0..pnum {
					
					println!("{}: {}", cats[gcati], projs[gcati][i]);
				}
				//for i in [0..pnum] {
					//println!("", , projs[gcati][i]);
				//}
		}
	}
	
	//println!("Generating `{}` projects in the category `{}` with showall: {}", pnum, pcat, pall);
}


