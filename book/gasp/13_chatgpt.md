# NPCとの会話をAIにする

NPCなどの会話や操作を音声とAIで実行します。

- [RuntimeSpeechRecognizer](https://github.com/gtreshchev/RuntimeSpeechRecognizer)
- [RuntimeAudioImporter](https://github.com/gtreshchev/RuntimeAudioImporter)
- [ChatGPT](https://platform.openai.com/docs/api-reference/introduction)
- [elevenlabs](https://elevenlabs.io/docs/api-reference/text-to-speech)

具体的な仕組みとしては、まずEnterを押すと、音声を読み取ります。読み取った音声を文字に変換し、それをchatgptに渡します。回答を再び音声に変換し、NPCと会話できるというシステムです。

<iframe src="https://blueprintue.com/render/i95n84w5/" scrolling="no" allowfullscreen style="width:100%;height:400px"></iframe>

```sh
$ curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gpt-4o-mini",
    "messages": [{"role": "user", "content": "Your question here"}],
    "temperature": 0.7
  }'
```

```sh
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

## 問題点

chatgptやelevenlabsは使用制限、あるいは課金が必要になります。

プレイヤーがNPCと会話する回数は初回のみのケースが多いと予想され、回数を制限することで、人気ゲームでこのような仕組みを実装しても、初月以降の負担はほぼ発生しないと予測されます。

しかし、hackされ、NPCとの会話でスパムを送信されることで負荷が増大する危険性があります。

したがって、このような仕組みは本番のゲームに実装すべきか難しいところです。

メリットとしては会話がプレイヤーごとに変わってくること。開発者がテキストを用意しなくても良いところが挙げられます。デメリットは制御できないところです。

これをlocalで動かすllmを使用することで多少の危険は避けられます。

今後、ゲームはどのような方向に進んでいくのでしょうか。NPCはAIが担当することになっていくのかもしれません。

