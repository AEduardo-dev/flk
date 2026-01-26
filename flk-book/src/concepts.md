# Core Concepts

flk is built around a few core concepts that help you manage your development environments effectively. Understanding these concepts will help you make the most out of flk.

## Projects

A project in flk is a directory that contains a `flake.nix` file. This file is the entry point to the environment for that project, including dependencies, custom commands, and environment variables.

When you navigate to a project directory and activate the flk environment, flk reads the `profile.nix` file and sets up the environment accordingly, using the files located in the `.flk/profiles/` directory.

## Profiles

Profiles are a way to manage different sets of dependencies and configurations for your projects. Each profile corresponds to a specific `profile.nix` configuration and is stored in the `.flk/profiles/` directory. You can switch between profiles using the `switch` command (given you set up the hook for your shell accordingly), allowing you to easily change your development environment based on the project you're working on.

## Custom Commands

Custom commands are user-defined scripts that can be added to your flk environment. These commands are defined in the `profile.nix` file and can be executed directly from the command line when the flk environment is activated. This allows you to create project-specific tools and utilities that are easily accessible.

## Environment Variables

Environment variables are key-value pairs that can be set within your flk environment. They are defined in the `profile.nix` file and are automatically loaded when you activate the flk environment for a project. This allows you to manage project-specific configurations, such as API keys or database URLs, without affecting your global environment.

> **Warning**
> CAUTION: Be careful not to store sensitive information in plain text within your `profile.nix` file, especially if the project is shared or version-controlled.

## Lock Files

Lock files are used to ensure that the dependencies for your flk projects remain consistent across different environments and over time. The lock file records the exact versions of dependencies used in your project, allowing you to reproduce the same environment later or on different machines. You can manage the lock file using the `flk lock` together with `flk update` commands.

## Overlays

Overlays are a powerful feature in flk that allow you to customize and extend the Nix package set used in your projects. By defining overlays in your environment, your profiles can modify existing packages or add new ones, tailoring the environment to your specific needs. This is particularly useful for projects that require specific versions of packages or custom builds, and is tightly integrated with the flk's lock file management to ensure consistency.

## Shell Integration

flk integrates with your shell to provide a seamless experience when working with different projects. By setting up shell hooks, flk can expose a set of convenient commands that allow you to switch between profiles, and hot reload them. This integration enhances productivity by making it easy to work within the context of your flk-managed projects without the need to exit and re-enter your shell environment on every change.

## Direnv Integration

flk can also integrate with `direnv`, a popular tool for managing environment variables based on the current directory. By using the `flk direnv init` command, you can generate a `.envrc` file that automatically loads your flk environment whenever you enter the project directory. This integration simplifies the workflow by ensuring that your development environment is always correctly set up without manual intervention.

Flk provides a set of commands `flk direnv` to help you manage this integration, making it easy to set up and maintain your development environments with `direnv`.

## Container Exporting

flk supports exporting your development environment to container formats like Docker and Podman. This feature allows you to create portable and reproducible environments that can be easily shared and deployed across different systems. By using the `flk export` command, you can generate container images that encapsulate your project's dependencies, custom commands, and environment variables. This is particularly useful if flk is not natively supported on your target system, or if you want to ensure consistency across development, testing, and production environments.
