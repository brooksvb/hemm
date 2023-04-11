# hemm

A minimal text-editor for distraction-free writing, able to integrate into any workflow.

## Philosophy

Writing is a multi-step process. The process and the methods we use can also change based on the context
of what type of document is being written and the goals of the writer.

Hemm aims to be a program for writing. _Just writing_. It aims to be devoid of distractions as much as possible.

The original aim I had for this program was a way to facilitate writing Morning Pages (link here?). The goal
of writing Morning Pages to be a focused time to just write whatever is on your mind before you start a day.

You might also find this program useful for long, focused writing sessions of longer content. An integrated
Pomodoro timer (planned feature) can allow you to work in multiple bursts of focused writing, with breaks in between.

### Hemingway mode

Hemingway mode disables all navigation and editing features, giving you only one option: _continue writing_.

To use Hemingway mode, pass the `--hemingway true` option, or specify this in your config file.

You might find this useful if you find that you constantly stop to fix typos, or rewrite a sentence multiple times. These
kinds of things can disrupt your chain and flow of thought. Enabling Hemingway mode encourages you to not get hung up
and to focus on allowing your thoughts to continue flowing.

It might be difficult to adjust to at first, as you find yourself making lots of typos that you frustratingly can't
fix when you see them. Over time, you should make less typos, and you will find ways to notate to yourself a correction when absolutely
necessary (e.g. I write something like "dstyopian -- dystopian"). More importantly, however, you will learn to stop worrying
about the errors and continue writing instead.

After you finish your writing session, you can open your file in a more featured editor, allowing you to more easily
correct your mistakes all at once.

## Features

-   [x] Open new or existing text files
-   [x] Periodically saves text in background
-   [x] "Hemingway-mode": Deletion and navigation disabled. You must only continue writing
-   [x] No fancy rendering, colors, formatting, previews
-   [x] Minimal UI elements (almost none)
-   [x] Multi-threaded for optimal performance and input capture

### Planned Features

-   [ ] Failsafe mechanism to save latest file version to backup if error occurs
    -   [ ] Error recovery screen to allow user to retry save, or copy text to clipboard
-   [ ] Set an optional timer to remind you when your writing session is done
-   [ ] Pipe output to another program for more flexible workflow and scripting
-   [ ] Integrated Pomodoro timer for extended writing sessions with breaks (using [porsmo](https://docs.rs/crate/porsmo/latest))
-   [ ] Implementation of additional standard navigation and editing shortcuts
    -   [ ] ctrl+backspace|delete - Delete word at a time
    -   [ ] ctrl+arrow - Navigate word at a time
    -   [ ] pgup|pgdown
    -   [ ] ctrl+enter - Open new line above current line
-   [ ] Copy-paste functionality (terminal shortcuts shift+ctrl+c|v should work)

#### Config

-   [ ] Load user configuration file to set default options
-   [ ] Change margin size
-   [ ] Optionally dim inactive line of text
-   [ ] Optionally underline active line of text
-   [ ] Define config "presets" for easier re-use (eg. --preset=morning, --preset=book)

### Out-of-scope Features

These features are out-of-scope for the goals of Hemm and will not be implemented. You should seek these
features in a different program if you need them.

-   Spellcheck
-   Writing suggestions
-   Syntax highlighting
-   Markdown rendering or rendering of any other format

## Usage

### Basic

```sh
hemm -h # Show help
hemm --help # Show extended help

hemm <filepath>
hemm --hemingway true <filepath> # Write in hemingway mode
```

### Workflow Examples

Hemm should be flexible enough that it can integrate into different semi-automated or automated workflows. Instead
of making assumptions about how to handle your files and every use-case, Hemm focuses on basic features, and expects
you to use other scripting tools to do anything more complex with the output.

If there is a use-case that you think Hemm does not work well for, but should, please open an issue.

Write a file with today's date to a directory (e.g. `~/Documents/Morning Pages/04-06-23`):
`hemm -d ~/Documents/Morning\ Pages $(date +%m\-%d\-%y)`

> [!warning] Editing Files While Open In Hemm
> Hemm has its own buffer of the file contents. If the file is changed externally, it will not
> get those changes, and will overwrite them the next time Hemm saves.
>
> It might be that you'd like to write a chunk of a text file, and then go back and edit it. While
> Hemm does not aim to provide advanced editing and proofreading features like spellcheck, word suggestion,
> or grammar analysis, you may like to use these features available in a different program.
>
> If you do so, it is important to close the Hemm editor before opening the file in another program.
> Re-opening the file with Hemm after you make your edits will allow you to resume at the end of the file.
