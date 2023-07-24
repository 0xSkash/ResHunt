# ResHunt

ResHunt is a command-line tool written in Rust to find unused [R.swift](https://github.com/mac-cain13/R.swift) resources in Xcode projects. It scans your project directory for resource files like strings and then checks each resource's usage in the codebase. Any unused resources are identified and reported.

## Features

- Scans iOS projects directory to collect resource files.
- Analyzes the resource usage in the codebase to identify unused resources.
- Outputs a list of unused resources for easy cleanup.

## TODO

- Support for other resources
- Support for other key formatting (only dots as delimiter supported)
