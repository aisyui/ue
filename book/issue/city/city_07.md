# 表示されなくなったサイト

昔は表示され操作もできたいくつかのサイトがweb browser pluginで操作も表示もされない現象が発生しています。ずっと待機中になります。

- https://web.syu.is/profile/ai.syu.is
- https://eyes.nasa.gov/apps/solar-system

一つは自分が管理しているサイトですが、cloudflareを使用しています。おそらく、cloudflareのセキュリティが動作しているのだと思います。web browserのbrowser versionが低いことも要因の一つかもしれません。

```json
{
  "time": "2024-10-20T08:00:00.00Z",
  "id": "1",
  "remote_ip": "ipv6",
  "host": "web.syu.is",
  "method": "GET",
  "uri": "/",
  "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) ++UE5+Release-5.4-CL Chrome/90.0.4430.212 Safari/537.36",
  "status": 200,
  "error": "",
  "latency": 0,
  "latency_human": "1.0ms",
  "bytes_in": 0,
  "bytes_out": 0
}
```

## 今のところうまく表示されているサイト

なぜか`solar-system`と同じものを使用している`asteroids`は表示されます。そのうち制限されるかもしれませんが。それともサイトの作りでしょうか。

- https://eyes.nasa.gov/apps/asteroids/#/planets/earth
