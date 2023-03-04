# rust-any2feed

A pluggable localhost server for convert any source to rss

Idea from [goutsune/unko](https://github.com/goutsune/unko), [goutsune/mewe-wrapper](https://github.com/goutsune/mewe-wrapper)


> <picture>
>  <source media="(prefers-color-scheme: light)" srcset="https://github.com/Mqxx/GitHub-Markdown/blob/main/blockquotes/badge/light-theme/warning.svg">
>  <img alt="Warning" src="https://github.com/Mqxx/GitHub-Markdown/blob/main/blockquotes/badge/dark-theme/warning.svg">
> </picture><br>
>
> The project is being written for learning rust and is not yet ready for users

## Features

For mvp  (minimal viable product)

* Sources:
    - [x] mewe
    - [x] telegram channels
      - [x] from public preview channel like https://t.me/s/bestogirl
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

## Usage

### Help
```shell
./any2feed help
./any2feed help run
```

### Run server

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


## Similar projects

* [goutsune/unko](https://github.com/goutsune/unko)
* [goutsune/mewe-wrapper](https://github.com/goutsune/mewe-wrapper)
* [stefansundin/rssbox](https://github.com/stefansundin/rssbox)
* [raspi/SimpleRedditRSSBot](https://github.com/raspi/SimpleRedditRSSBot)
* [c3kay/hoyolab-rss-feeds](https://github.com/c3kay/hoyolab-rss-feeds)
* [pink-red/rss-exhentai-watched](https://github.com/pink-red/rss-exhentai-watched)
* [kthchew/ao3-rss](https://github.com/kthchew/ao3-rss)
* [SkYNewZ/youtube-subscription-feed](https://github.com/SkYNewZ/youtube-subscription-feed)
