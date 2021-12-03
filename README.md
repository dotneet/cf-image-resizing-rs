# Getting Started

Cloudflare WorkersとRustを使って画像をリサイズするサンプルです。

 - バグがあるのでworkerをこの[PR](https://github.com/cloudflare/workers-rs/pull/81)をマージして独自ビルドする必要があります。(2021/12/4現在)
 - Cache APIは未実装。[PR](https://github.com/cloudflare/workers-rs/pull/67)が出てるのでマージして独自ビルドすれば使えます。サンプル内では未使用です。
 - JPEGは jpeg_rayon という並列処理の機能を外さないと動かないことに注意が必要です。[関連Issue](https://github.com/image-rs/image/issues/879)

## Usage 

```bash
# compiles your project to WebAssembly and will warn of any issues
wrangler build 

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```

## 参考

 - [WebAssembly で画像のリサイズ処理をやってみたら JavaScript + Canvas API より遅かった話](https://qiita.com/yokra9/items/f9e98a9b47fe2d1234b0)
