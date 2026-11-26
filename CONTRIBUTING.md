# Contributing to TB371FC-nobscur

[Русская версия](CONTRIBUTING.ru.md)

Follow the [code quality agreement](CODE_QUALITY.md) and write [Conventional Commits](COMMIT_CONVENTIONS.md) in English.

Keep TB371FC wiring and configuration in the relevant [component](workspaces/components/README.md). Put changes to reusable drivers and tools in their own repositories. Publish the dependency commit and update its submodule pointer when the board project needs that revision.

Run the checks for the component you changed. Compile Device Tree fragments with the board DTS. For kernel patches, check that the patch series applies and build the affected kernel targets. In the pull request, describe the behavior you checked and identify the kernel and hardware if you tested on a device.

Use synthetic values in shared examples and logs. Keep device-specific identifiers, firmware dumps, and local notes out of commits.
