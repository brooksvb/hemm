# hemm

A minimal text-editor for distraction-free writing, able to integrate into any workflow.

## Features

-   Open new or existing text files
-   Periodically saves text in background
-   "Hemingway-mode": Deletion and navigation disabled. You must only continue writing
-   No fancy rendering, colors, formatting, previews
-   Minimal UI elements
-   Multi-threaded for optimal performance and input capture

### Planned Features

-   Load user configuration file for options
-   Failsafe mechanism to save latest file version to backup if error occurs
-   Set an optional timer to remind you when your writing session is done
-   Pipe output to another program for more flexible workflow and scripting

## Basic Usage

```sh
hemm -h
hemm <filepath>
hemm --hemingway true <filepath> # Write in hemingway mode
```

## Workflow Examples

TODO
