## v0.1.0-a0 (2023-03-13)

### Feat

- **any2feed**: add --version to cli
- **telegram**: media proxy complete #2
- **telegram**: media proxy logic for workaround invalidated media; #2
- **any2feed**: implement cli
- **telegram**: add opml; config fix(telegram): fill feed author
- **telegram**: debug mode feed, id non unique for debug
- **telegram**: handle voice; gif mode fix(telegram): round video issue
- **telegram**: first view import from telegram preview
- **mewe-api**: format user mention
- **mewe-api**: best way error handling
- improve config; add config example;
- improve rendering ref_post
- **mewe**: move routes to mewe importer; add config
- **mewe**: render poll, file, gif post content
- **mewe**: add group/user feed; generate opml with group/user
- add path patterns
- mewe api fetch contacts, groups, feed for contact and group
- add opml generation

### Fix

- **telegram**: forwarded from private user/channel
- **reqwest-mozilla-cookie**: format cookies-txt for Firefox
- **mewe**: fix logic unique id only in  group/user/myworld
- **mewe**: fetch next_page error
- add empty title for empty posts for fix loading feed issue in t*bird
- add lost opml generation structure
- run clippy --fix

### Refactor

- rename from importer[s] to feed_source[s] #5
- **http-server**: simple error handling
- routes, tests, up readme and contributors docs
- reorganizes deps
- run clippy --fix
- Refactoring code structure
