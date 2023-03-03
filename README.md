# rust-any2feed

A pluggable localhost server for convert any source to rss

Idea from [goutsune/unko](https://github.com/goutsune/unko), [goutsune/mewe-wrapper](https://github.com/goutsune/mewe-wrapper)


> <picture>
>  <source media="(prefers-color-scheme: light)" srcset="https://github.com/Mqxx/GitHub-Markdown/blob/main/blockquotes/badge/light-theme/warning.svg">
>  <img alt="Warning" src="https://github.com/Mqxx/GitHub-Markdown/blob/main/blockquotes/badge/dark-theme/warning.svg">
> </picture><br>
>
> The project is being written for learning rust and is not yet ready for users

# Feature

For mvp  (minimal viable product)

* Sources:
    - [x] mewe
    - [x] telegram channels
      - [x] by preview channel like https://t.me/s/bestogirl
      <!-- - [ ] by telegram client api -->
    - [ ] danbooru/gelbooru
    - [ ] pixiv
    - [ ] twitter
    - [ ] tumblr
    - [ ] pikabu
    - [ ] dtf
    - [ ] vk
    - [ ] 2ch
* [x] CLI
* [ ] Configure via env
* [ ] Pluggable interface
* [ ] Cache storage
* [x] Config

# Usage

Help
```shell
./any2feed help
./any2feed help run
```

Run server
```
Usage: any2feed --config <CONFIG> run [OPTIONS]

Options:
  -p, --port <PORT>        Server listen port
      --threads <THREADS>  Server num threads
```

CLI params overrides config param
```shell
./any2feed --config ./any2feed.config.toml run [--port 12345]
```
