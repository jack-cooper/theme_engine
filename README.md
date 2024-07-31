# Theme Engine

_Theme Engine_ is a delightful piece of software adding a sporadic sprinkle of variety to a Discord
server.

## Project Structure
_Theme Engine_ consists of 2 components:
- A web server, written using [axum](https://github.com/tokio-rs/axum)
- An [SQLite](https://www.sqlite.org/) database

## Setup
_Theme Engine_ should run out of the box with only 1 piece of configuration required, a `.env` file
which it will search for within the directory it was run from. This should have the following
format:
```bash
DATABASE_URL=sqlite:theme.db
PORT=3000
THEMELORD_URL=$DISCORD_WEBHOOK_URL
```

- `PORT` can be any port of your choosing
- `$DISCORD_WEBHOOK_URL` should be replaced with the URL for a
[Discord webhook](https://discord.com/developers/docs/resources/webhook). 

## Endpoints

The _Theme Engine_ web server exposes 5 endpoints:
- `/` (`GET`) - Lists all upcoming themes
- `/active` (`GET`) - Displays the currently active theme
- `/archive/culled` (`GET`) - Lists all themes that were culled and never used
- `/archive/previous` (`GET`) - Lists all themes that were previously used
- `/invoke_themelord` (`POST`) - Selects a new theme

## How to Use
To run _Theme Engine_, simply copy the files from the GitHub release over to a Linux server of your
choosing, and run the contained binary. Selecting a theme can then be done by running a cron job
to fire off a request to the `invoke_themelord` endpoint, using e.g. `curl`.

## Versioning
As a binary release, semantic versioning doesn't make the most sense for _Theme Engine_, but in
the interest of using something reasonably understandable, and which Cargo has support for, I've
elected to stick with it anyway. Version bumps for _Theme Engine_ can be understood as follows:

- Major version bump, e.g. `v1.0.1` => `v2.0.0`: Significant restructure of the web server
(unlikely to ever happen)
- Minor version bump, e.g. `v1.0.1` => `v1.1.0`: Any other code changes
- Patch version bump, e.g. `v1.0.1` => `v1.0.2`: Changes to themes, i.e. _only_ `theme.db` was
modified