# rust-int-join-bench

## 概要

Rustで`Iterator<Item=usize>`から数値を連結した文字列を生成する際のパフォーマンスの測定。

詳細は以下:

[Rustで数値を連結した文字列を作るときはItertools::joinが速い](https://qiita.com/shortheron/items/27b9030ab8ac33875ac7)


ベンチマークのソースコードは `benches` 配下にあります。

## 実行方法

```
cargo bench
```