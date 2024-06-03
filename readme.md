# yui

- [aiverse](https://git.syui.ai/ai/ue/wiki/project) project
- [aiue](https://git.syui.ai/ai/ue/wiki/system) system

see [ue.json](https://git.syui.ai/ai/ue/src/branch/main/ue.json)

## log

|version|commit|
|---|---|
|v0.1 β|world create|
|v0.2 β|support web|
|v0.3 β|support vmc|
|v0.4 β|support at|
|v0.5 β|support battle|

## at

the player data is stored in the pds.

```sh
├── [yui.syui.ai]
│   ├── ai.syui.game.user
│   │   ├── lv
│   │   ├── hp
│   │   └── coin
│   └── ai.syui.game.login
│       ├── login <bool>
│       ├── updatedAt
│       └── username
└─── [user.bsky.social]
     └── ai.syui.game
        ├── account <at://yui.syui.ai...>
        └── createdAt
```

```sh
# https://git.syui.ai/ai/at/src/branch/main/lexicons/ai/syui/game
$ ./at.zsh u at://did:plc:4hqjfn7m6n5hno3doamuhgef/ai.syui.game.user/syui
{
  "uri": "at://did:plc:4hqjfn7m6n5hno3doamuhgef/ai.syui.game.user/syui",
  "cid": "bafyreigijd4vonyzgjkzotrbtq5j5gyoecokoij3u7jw4sqnx6wkh7attq",
  "value": {
    "did": "did:plc:uqzpqmrjnptsxezjx4xuh2mn",
    "$type": "ai.syui.game.user",
    "aiten": 0,
    "limit": false,
    "login": false,
    "gender": "male",
    "handle": "syui.ai",
    "character": {
      "ai": {
        "hp": 9,
        "lv": 1,
        "exp": 0,
        "img": "https://cdn.bsky.app/img/feed_thumbnail/plain/did:plc:4hqjfn7m6n5hno3doamuhgef/bafkreie34pjuc6coenzcdwrgrh4fbacq7bkhsz263g5vpbsqxwaz37kkwy@jpeg",
        "mode": 0,
        "rank": 0,
        "group": "origin",
        "attach": 0,
        "season": 0,
        "critical": 1,
        "critical_d": 0,
        "attach_post": 14102
      }
    },
    "createdAt": "2024-11-29T21:34:27.833Z",
    "updatedAt": "2024年12月8日 11:25:17 GMT"
  }
}
```

## service

|title|url|
|---|---|
|game|https://ue.syui.ai|
|live|https://live.syui.ai|
|chat|https://o.syui.ai|

## support

`windows 64bit`

|title|spec|
|---|---|
|cpu|AMD Ryzen 7 5700X|
|memory|32GB / DDR4-3200 DIMM (PC4-25600)|
|gpu|GeForce RTX 4060Ti 8GB|
|storage|1TB M.2 NVMe SSD|
