# Contributing to `sachy.rs`

This project is a personal project, and contributions are not expected from
external contributors. The guidelines provided here are solely for personal
reference.

## Commit Style

Each commit message should have the following format:

```
<type>(<scope>): <subject>

<description>

<footer>
```

The footer should contain a `Closes` statement if the commit closes an issue or
pull request.

Types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code (white-space,
formatting, missing semi-colons, etc.)
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools and libraries such as
documentation generation

Scopes:
- `core`: Changes to the core library
- `docs`: Changes to the documentation
- `tests`: Changes to the test suite
- `build`: Changes to the build system
- `deps`: Changes to dependencies
- `misc`: Miscellaneous changes

## Branches

Branches should be named in the following format:

```
<type>/<scope>/<description>
```

The types and scopes are the same as the commit types, with the type addition of
`hotfix` for hotfix branches. Hotfix branches should be named in the following
format:

```
hotfix/<description>
```

Hotfix branches should be branched off of the `main` branch and merged into the
`main` branch.

## Pull Requests

The template for pull requests is provided in the
[`PULL_REQUEST_TEMPLATE.md`](PULL_REQUEST_TEMPLATE.md) file.

Pull requests should be named in the following format:

```
<type>(<scope>): <subject>
```

The types and scopes are the same as the commit types, with the type addition of
`hotfix` for hotfix pull requests. Hotfix pull requests should be named in the
following format:

```
hotfix: <subject>
```

## Code Style

The code style is enforced by `rustfmt` and `clippy`. The `rustfmt`
configuration is provided in the [`rustfmt.toml`](rustfmt.toml) file. The
`clippy` configuration is provided in the [`clippy.toml`](clippy.toml) file.

## Testing

All code should be tested. The test suite can be run with `cargo test`. The test
suite is run on every pull request by GitHub Actions.