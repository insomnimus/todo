# todo

Todo is a simple command line note keeping application.

# The Motive

Todo is very simple and definitely not one of the flashy todo apps. But there're reasons for that:

-	It doesn't have any distractions and just gets to the point. That means less time spent on configuring everything.
-	Note taking and viewing is a command away so it's very convenient to just scratch something or to refresh yourself on your ideas.
-	It won't remind you to do your chores, instead it waits for you to want to be productive. This way, you are more efficient at your work and you procrastinate less.

# Features

Todo's simplicity doesn't make it completely featureless, here is what todo offers:

-	Hooks: you can configure any number of pre/post hooks, depending on the command you ran. Hooks are plain commands you put in the todo's configuration file.
-	Shell completions: todo comes with shell completions.
-	Tags: you can put tags to your todos.
-	Importance level: You can set an importance level to any todo.
-	Filters: you can filter your todos by the name, tag, index or importance level.
-	Coming soon: git integration (for now you can achieve similar results with the hooks).
-	Coming soon: Editor integration.

# Installation

Todo is written in rust and is tested with cargo version 1.54.0. You will need an up to date rust environment to compile it.

Todo is not yet published on crates.io but it is planned, meanwhile you can follow any of the following methods to install it.

### Installation with git and cargo

```sh
git clone https://github.com/insomnimus/todo
cd todo
git checkout main
cargo install --path .
```

The shell completion files will be written to the project root after cargo installs todo on your system.

### Installation with cargo only (no shell completions)

`cargo install --git https://github.com/insomnimus/todo --branch main`

# Usage

The usage is pretty straightforward:

```sh
# create a new todo
todo new "title" "body"
# create a new todo with tags and an importance level
todo new "title" "body" --tag=tag1,tag2,tag3 --level=2
# view 5 most recent todos
todo
# view all your todos
todo list
# view todos by their tag
todo list -t my_tag
# search for todos by their title, also using glob patterns
todo list "do *"
# remove todos by the title, again using glob but glob is not a requirement
todo remove "mail *"
# remove a range of todos (start from the 5th most recent, ending with the 10th most recent)
todo remove -i 5:10
# view todos in reverse order
todo list -i=-1:0
```

There are of course more things you can do, please run `todo --help` for the full usage.

# Config File Syntax and Hooks

A tutorial will be added to this readme soon.
