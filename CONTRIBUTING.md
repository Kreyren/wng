# Commit guideline

## Keywords

`feat` : Feature added
`fix` : Fixed a bug
`docs` : Added docs
`refactor` : Refactored code

## Style

\<type>([scope]): \<message> | [refs] [advanced description]

### Examples :

`docs(lang): Added polish language | #256`

`feat(install): Added an installation system | #22 #23`

`refactor(creation): Optimized project creation`

# Pull requests

## Badges

Your pr has to contain the appropriated badge :

Fix : ![fix](https://img.shields.io/badge/PR_kind-Fix-critical)

Feature : ![feat](https://img.shields.io/badge/PR_kind-Feat-success)

Docs : ![docs](https://img.shields.io/badge/PR_kind-Docs-informational)

Refactor : ![docs](https://img.shields.io/badge/PR_kind-Refactor-important)

## Style

Pull requests have to fit this style :

```
## Title

![kind](link_for_corresponding_badge)

### Description :


### Corresponding issues :
```
