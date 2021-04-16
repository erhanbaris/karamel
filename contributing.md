# Contributing

[![open issues](https://badgen.net/github/open-issues/erhanbaris/karamel?label=issues)](https://github.com/erhanbaris/karamel/issues) [![help welcome issues](https://badgen.net/github/label-issues/erhanbaris/karamel/help%20welcome/open)](https://github.com/erhanbaris/karamel/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+welcome%22) [![good first issue](https://badgen.net/github/label-issues/erhanbaris/karamel/good%20first%20issue/open)](https://github.com/erhanbaris/karamel/issues?q=is%3Aopen+is%3Aissue+label%3A%22beginner+friendly%22)

**Contents**

* [Welcome](contributing.md#welcome)
  * [Prerequisites](contributing.md#prerequisites)
* [Requesting Features](contributing.md#requesting-features)
  * [Language Requests](contributing.md#language-requests)
* [Reporting Issues](contributing.md#reporting-issues)
* [Fixing Issues \(PRs\)](contributing.md#fixing-issues-prs)
  * [In a nutshell](contributing.md#in-a-nutshell)
  * [Build and Test](contributing.md#build-and-test)

## Welcome

Hello and welcome to **Karamel**. We are making **Turkish programming language**, but you probably knew that already. If you are considering contributing to **Karamel** this document will be hopefully be a helpful resource.

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

**You do not have to be a programmer.** There are many ways to contribute::

* Hang out on our [Discord](https://discord.gg/8ymtm9XPyQ) and help answers questions as they come up
* Report [new issues or bugs](https://github.com/erhanbaris/karamel/issues/new/choose) or join the existing discussion on open issues
* Submit pull requests to resolve issues
* Improve our documentation to better explain all the things to all the peoples

### Prerequisites

* To help answer questions on issues or contribute on Discord you need only be friendly.
* To work on the core language engine you'll need to know Rust-Lang.
* To work on documentation you need to be ready and willing to document things.

## Requesting Features

Feature requests are always welcome. If the feature doesn't belong in the core library then we're always happy to suggest how you might go about developing a plug-in.

If you're thinking of contributing a feature first open an issue to discuss whether the feature belongs in core vs a plug-in. Often this is a great way to get implementation tips or links to prior discussions on the topic often with additional context, etc.

## Reporting Issues

If you find a bug or think of an improvement, feel free to [open an issue](https://github.com/erhanbaris/karamel/issues/new/choose).

## Fixing Issues \(PRs\)

If you feel comfortable with the [prerequisites](contributing.md#prerequisites), you can grab any issue marked ["good first issue"](https://github.com/erhanbaris/karamel/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22). Or feel free to jump in with thoughts or comments on any of the more complex issues.

### In a nutshell

If you're new to contributing to open-source, have a look at [this GitHub Guide](https://guides.github.com/activities/forking). It explains the general process of GitHub collaboration.

Karamel is developed in Rust-Lang, so you'll need the usual suspects: [Rust-Lang](https://www.rust-lang.org), git, etc. You'll likely start by forking the repository on GitHub and then cloning it locally.

1. Fork this project on GitHub.
2. Clone it locally `git clone git@github.com:username/Karamel.git`.
3. Create a work branch \(`git checkout -b my-branch`\).
4. Commit your changes \(`git commit -m 'my changes'`\).
5. [Build and Test](contributing.md#build-and-test)
6. Push the branch \(`git push origin my-branch`\).
7. Open a Pull Request from your fork back to this repository.

### Keep in Mind

Please open a new issue before your PR \(or join the discussion on the existing issue\), so we can explore and discuss the topic at hand. Your time is important, and we need to be sure it's well-spent.

_Before_ you start coding, keep these tips in mind:

* You should usually add markup tests \(ie. whenever you've made a significant grammar change or fixed a bug\). Simply adding `keywords` can be an exception to this rule.
* Change only what needs to be changed; don't re-lint or rewrite whole files when fixing small bugs
* Linting or major re-organization needs a dedicated commit

_After_ you make your changes, we have some housekeeping tasks for you - like updating the [changelog](https://github.com/erhanbaris/karamel/blob/main/CHANGES.md). The PR template will be your guide.

### Build and Test

When contributing a PR \(that doesn't make any specific changes to browser features\) it's usually sufficient to build and test only the Node.js build. Our CI process will guarantee that the browser build is still green.

Building the Karamel:

```text
cargo build --release
```

Testing the Karamel:

```text
cargo test --all
```

