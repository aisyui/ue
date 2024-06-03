# Unreal Engine 5.4 | 初めてのゲーム制作、世界を作る

## vrm4u

キャラクターを表示しよう。

## game animation sample

今後はこの形式が基本になりそう。

## city sample

最初に難易度と負荷を高くする。

## sky atmoshpere + volumetric cloud

`dynamic volumetric sky -> ultra dynamic sky`

## whisper + chatgpt + elevenlabs

- whisper : RuntimeSpeechRecognizer

```sh
# perplexity.ai
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

## ue vs unity

ueは手順通りやっても動くことは稀。つまり、動かない。そのため情報も少ない。unityがおすすめ。

