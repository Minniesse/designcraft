# KatWork fork of DesignCraft

This repository is the KatWork fork of DesignCraft (https://github.com/storytold/designcraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on DesignCraft by the ArtCraft team (https://github.com/storytold/designcraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `cdd650ccddcd945bc3bd050f40299955a7dd1c4b`
- Upstream `storytold/designcraft` head at that time: `cdd650ccddcd945bc3bd050f40299955a7dd1c4b`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/designcraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.designcraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The nightly channel's messages and tests that name ArtCraft Launcher (`build_flavor.rs`, `main.rs`, `updates_ui.rs`), the
  Windows install-policy check's Launcher case, and the Flatpak permission for the launcher's version folder
  (`xdg-data/ArtCraft/versions`): they name a separate program and its folders, the upstream nightly builds' updater; KatWork
  builds no nightly.
- The upstream release workflow's secret name (`release.yml`): upstream CI configuration, unchanged; KatWork does not run it.
- The "Based on" line in the About window's code (`crates/ui-egui/src/about.rs`).

### Other text this fork changed

- The About window drew the ArtCraft mark (an inline copy of its outline, `MARK_PATH`) above the app's name; the mark is gone
  and the app's name, in its own heading, now heads the window. The start screen's community card drew the mark beside
  "Join the ArtCraft community"; it is now a project card with the app's name as plain text (the card title's size and
  weight) and the GitHub and issue-tracker links. The app bar's Discord button, the Help menu's Discord, ArtCraft website and
  ArtCraft app-page items, and the About window's sibling-app buttons are removed.
- `app.links` returns `github` and `issues`; `help.discord`, `help.website`, `help.appPage` and `help.app` are no longer
  commands. The MCP server's instructions and `websiteUrl`, the CLI's `links` subcommand and its usage footer name the GitHub
  repository and its issues.
- `ASSETS.md`, `assets/app-icon/LICENSE.txt` and `assets/app-icon/README.md` said the app icon was drawn "in ArtCraft"; they
  now say it was drawn by the project owner (author, source and licence unchanged). The user docs (`docs/user/`) lose their
  Discord and website links and the ArtCraft Launcher install section.

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.svg`: removed

Every other image of the fork base, each kept:

- `assets/app-icon/designcraft-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft-macos-512.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft-nightly-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft-nightly.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft-nightly.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft-nightly.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft-small.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/designcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.designcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.designcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/24x24/apps/ai.storyteller.designcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.designcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.designcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.designcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.designcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.designcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.designcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `crates/render/tests/golden/drop-shadow-spread.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/render/tests/golden/drop-shadow-text.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/render/tests/golden/drop-shadow.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/render/tests/golden/feather.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/render/tests/golden/inner-shadow.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `crates/render/tests/golden/outer-glow.png`: kept: render test golden image (viewed: no ArtCraft mark)
- `docs/images/page-2.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/page-3.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/page-4.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/page-cover.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/ui-spread.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `packaging/macos/dmg/background.svg`: kept: macOS disk-image background, original artwork (the app name and icon; viewed as its TIFF rendering: no ArtCraft mark)
- `packaging/macos/dmg/background.tiff`: kept: macOS disk-image background, original artwork (the app name and icon; viewed: no ArtCraft mark)
