---
theme: eloc
class: text-center
highlighter: shiki
lineNumbers: false
info: |
  ## Slidev Starter Template
  Presentation slides for developers.

  Learn more at [Sli.dev](https://sli.dev)
drawings:
  persist: false
transition: slide-left
title: Unreal Engine 5.5 | aiue
---

# `aiue`

物語は空と海に囲まれた西の都(みやこ)からはじまる...

---

## 配信で使える最新技術の紹介

### `unreal engine`

- vrm4u, vmc, livelink, streaming
- chatgpt, atproto
- `ai` + `ue`

---

## `unreal engine`

- ue 5.5.0p 
- ue 5.4.4

---

## `vrm4u`

キャラクターを表示しよう

---

`vmc`と`livelink`で体の動きを反映

- vmcはABP
- livelinkはCBP

---

## `web browser`

WBPからwebを使おう

---

- widget3dをworldに表示させると画質が悪いので`EngineMaterials/Widget3DPassThrough`以外のmaterialを使います

<iframe src="https://blueprintue.com/render/-49_059w/"></iframe>

https://blueprintue.com/blueprint/-49_059w/

---

## `pixel streaming`

webでゲーム配信や操作ができる

```sh
$ git clone https://github.com/EpicGamesExt/PixelStreamingInfrastructure
$ cd ./PixelStreamingInfrastructure/SignallingWebServer/platform_scripts/cmd/
$ ./Start_SignallingServer_nopublic.ps1
```

---

## `atproto`

blueskyが使っているprotocol

---

## `game animation sample`

キャラクターの基本操作をカスタマイズ

---

## `city sample`

人や車が動く最先端の街

---

## `ultra dynamic sky`

- `sky atmoshpere` + `volumetric cloud`

---

## `whisper` + `chatgpt` + `elevenlabs`

キャラ設定と会話

- whisper : RuntimeSpeechRecognizer

---

```sh
# perplexity.ai
$ curl -X POST "https://api.elevenlabs.io/v1/text-to-speech/VOICE_ID" \
     -H "xi-api-key: YOUR_API_KEY" \
     -H "Content-Type: application/json" \
     -d '{
           "text": "Hello world!",
           "model_id": "eleven_monolingual_v1",
           "voice_settings": {
             "stability": 0.5,
             "similarity_boost": 0.5
           }
         }' \
     --output output.mp3
```

---

@syui.ai

<br/>
<img src="https://yui.syui.ai/icon/ai.svg" width="50px">
<!--
<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/7a/Bluesky_Logo.svg/1200px-Bluesky_Logo.svg.png" width="30px">
-->
