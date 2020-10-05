# Agent Core
**[:fox: View in GitLab](https://gitlab.com/agent-assistant/core/)** - ["White paper"](https://telegra.ph/Agent-Draft-10-02) - Docs - Builds

Agent is a smart virtual assistant that does not communicate with the cloud to operate.
Advantages of this method include heightened privacy and faster response times.

This repository contains the core processing Agent uses. It includes basic code for Agent suggestions, Agent search, and Agent itself.

## Testing Agent
You can test agent by cloning the repository and running the following:
```zsh
cargo run -q -- --help
```
`--help` can be replaced with any argument that Help tells you.

Agent Core has three flavors of processing:
* Text suggestions, which can be triggered using `-s` or `--suggestions`
* Search suggestions, which can be triggered using `-S` or `--search`
* Agent responses, which can be triggered using `-q`, `--query`, or `--askagent`

An example query input for suggestions:
```zsh
cargo run -q -- -si "what time is it"
cargo run -q -- -si "what is the time"
# Fun fact: only "what time is it" is checked for in the `suggestions/time.rs` file!
```

## Contributing to Agent
While Agent is mirrored on GitHub, the main repository lies at [GitLab](https://gitlab.com/agent-assistant/core).

### Code Contributions
Create a Merge Request containing your changes. We use commit tags for core modules and some subjects (see below). Use all of them that apply at the front of your commit message, and none if none apply:
```
[sugg][ask] Add new plugin APIs
```
Available tags are:
* `[sugg]` for text suggestions
* `[ask]` for Agent responses
* `[search]` for search suggestions
* `[ci]` for build/repository automation
* `[mods]` for module-system related changes
* `[minor]` for minor corrections to the repository, such as a lockfile update or gitignore fix

`[skip ci]` is a magic tag that causes your commit not to trigger the CI (good for `[minor]` changes). It can be put below the summary, preferrably alongside an explanation if one is not implied/stated by the commit message.

By submitting a pull request, you agree to release your changes under the MIT license.

### Bug Reports
Find an issue in Agent? Report it! This helps Agent developers know what needs to be improved. You can also suggest features.

The Issues area is to the left, but you can also email us at:
```
incoming+agent-assistant-core-21534182-issue-@incoming.gitlab.com
```

### Build a Module
You can build Agent by building your own module! You can import it in your own Agent or at the command line.

Documentation and implementation for this are not yet available.

# AGENT IS A WORK IN PROGRESS.
**Not all features advertised here are available. In fact, most aren't.**