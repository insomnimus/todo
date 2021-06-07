use clap::{crate_version, App, AppSettings, Arg};

fn validate_index(s: &str) -> Result<(), String> {
    let vals: Vec<_> = s.split(':').collect();
    match vals.len() {
        0 => {
            Err("the syntax is START:END or just N, where START, END and N are integers".to_owned())
        }
        1 => s
            .parse::<isize>()
            .map(|_| ())
            .map_err(|_| format!("{} is not a valid number/ range pattern", s)),
        2 => {
            let (x, y) = (vals[0], vals[1]);
            if x.is_empty() && y.is_empty() {
                Ok(())
            } else if x.is_empty() {
                y.parse::<isize>()
                    .map_err(|_| format!("{}: not a number", y))
                    .map(|_| ())
            } else if y.is_empty() {
                x.parse::<isize>()
                    .map_err(|_| format!("{}: not a valid number", x))
                    .map(|_| ())
            } else if let (Ok(_), Ok(_)) = (x.parse::<isize>(), y.parse::<isize>()) {
                Ok(())
            } else {
                Err(format!("{}: invalid range expression", s))
            }
        }
        _ => Err("only 1 ':' is allowed in a range expression".to_owned()),
    }
}

fn validate_minmax(s: &str) -> Result<(), String> {
    let vals: Vec<_> = s.split(':').collect();
    match vals.len() {
		0=> Err("the syntax is MIN:MAX or just LVL, where MIN, MAX and LVL are integers between 0 and 255".to_owned()),
		1 => s.parse::<u8>().map(|_| ()).map_err(|_| format!("{} is not a valid number/ range pattern", s)),
		2=> {
			let (x, y)= (vals[0], vals[1]);
			if x.is_empty() && y.is_empty() {
				Ok(())
			}else if x.is_empty() {
				y.parse::<u8>().map(|_| ()).map_err(|_| format!("{}: not a valid number", y))
			}else if y.is_empty() {
				x.parse::<u8>().map(|_| ()).map_err(|_| format!("{}: not a valid number", x))
			}else if let (Ok(_), Ok(_)) = (x.parse::<u8>(), y.parse::<u8>()) {
				Ok(())
			}else{
				Err(format!("{}: invalid range expression", s))
			}
		}
		_=> Err("only 1 ':' is allowed in a range expression".to_owned())
	}
}

pub fn app() -> App<'static> {
    let app_where = App::new("where")
        .visible_alias("w")
        .aliases(&["wh", "which", "whereis"])
        .about("show the path of the todoes file");

    App::new("todo")
        .about("simple note tracker")
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::AllowNegativeNumbers)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .version(crate_version!())
        .subcommand(app_list())
        .subcommand(app_new())
        .subcommand(app_remove())
        .subcommand(app_where)
}

pub fn app_list() -> App<'static> {
    let app = App::new("list")
        .about("display notes")
        .visible_alias("l")
        .aliases(&["show", "display", "ls"]);

    let index = Arg::new("index")
        .short('i')
        .long("index")
        .about("expression to filter the result by index")
        .long_about(
            "an expression to filter the results by index
	syntax: start:end
	or: N
	start or end can be omitted
	last note has the index 0",
        )
        .validator(validate_index)
        .takes_value(true);

    let title = Arg::new("title")
        .about("filter results by their title")
        .long_about(
            "filter results by their title
	glob patterns are allowed and matching is case insensitive",
        );

    let lvl = Arg::new("lvl")
        .short('l')
        .long("level")
        .about("filter results by their importance level")
        .long_about(
            "filter results by their importance level
	syntax: MIN:MAX
	or: LVL
	MIN or MAX can be omitted",
        )
        .validator(validate_minmax)
        .takes_value(true);

    let tag = Arg::new("tag")
        .short('t')
        .long("tag")
        .takes_value(true)
        .about("comma separated list of tags to filter the results with")
        .long_about(
            "comma separated list of tags to filter the results with. tags are case insensitive",
        );

    app.arg(title).arg(index).arg(lvl).arg(tag)
}

pub fn app_remove() -> App<'static> {
    let app = App::new("remove")
        .visible_alias("r")
        .aliases(&["del", "delete", "rm"])
        .about("remove notes")
        .setting(AppSettings::ArgRequiredElseHelp);

    let title = Arg::new("title")
        .about("a glob pattern matching the note title")
        .long_about("a glob pattern matching the note title. matching is case insensitive");

    let index = Arg::new("index")
        .short('i')
        .long("index")
        .about("index of the note to remove")
        .long_about(
            "index of the note to remove
	syntax: START:END
	or N
	START or END can be omitted
	negative numbers are allowed (will count from the end of the list)
	the newest note will be index 0",
        )
        .takes_value(true)
        .validator(validate_index);

    let lvl = Arg::new("lvl")
        .short('l')
        .long("level")
        .takes_value(true)
        .about("remove notes matching the importance level")
        .long_about(
            "remove notes matching the importance level
	syntax: MIN:MAX or LVL
	MIN or MAX can be omitted",
        )
        .validator(validate_minmax);

    let tag= Arg::new("tag")
	.short('t')
	.long("tag")
	.takes_value(true)
	.about("space separated list of tags to remove notes having any of them")
	.about("space separated list of tags to remove notes having any of them. tags are case insensitive");

    app.arg(title).arg(index).arg(lvl).arg(tag)
}

pub fn app_new() -> App<'static> {
    let app = App::new("new")
        .about("take a note")
        .visible_alias("n")
        .aliases(&["create", "add"]);

    let title = Arg::new("title")
        .about("note title")
        .takes_value(false)
        .required(true);

    let body = Arg::new("body")
        .about("note body")
        .takes_value(false)
        .required(true);

    let tag = Arg::new("tag")
        .about("comma separated list of tags")
        .short('t')
        .long("tag")
        .takes_value(true)
        .long_about("comma separated list of tags. can be used to filter notes");

    let lvl = Arg::new("lvl")
        .short('l')
        .long("lvl")
        .about("importance level of the note")
        .long_about("importance level of the note (0..255). can be used to filter notes")
        .takes_value(true)
        .validator(|s: &str| -> Result<(), String> {
            s.parse::<u8>()
                .map_err(|_| {
                    format!(
                        "invalid importance level '{}': must be a number between 0 and 255",
                        s
                    )
                })
                .map(|_| ())
        });

    app.arg(title).arg(body).arg(tag).arg(lvl)
}
