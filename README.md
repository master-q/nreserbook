# 横浜市立図書館の予約件数を表示

## 0. セットアップ

* "API ダッシュボード | カーリル" https://calil.jp/api/dashboard/
* カーリルのAPIキーを得る

## 1. 蔵書情報の更新

```
$ git clone git@github.com:master-q/nreserbook.git
$ cd nreserbook
$ CALIL_APPKEY=カーリルのAPIキー cargo run update sample_booklist.md
+..........
```

## 2. 予約件数の表示

```
$ CALIL_APPKEY=カーリルのAPIキー cargo run show sample_booklist.md | sort -g
- / * "Compiling with Continuations" https://www.amazon.com/dp/052103311X
0 / * "サピエンス全史(上)文明の構造と人類の幸福" https://www.amazon.co.jp/dp/430922671X
1 / * "AIとSF (ハヤカワ文庫JA) : 日本SF作家クラブ: 本" https://www.amazon.co.jp/dp/product/4150315515/
49 / * "池田暁子の必要十分料理 : 池田暁子: 本" https://www.amazon.co.jp/dp/4798701882
75 / * "プロジェクト・ヘイル・メアリー 上 | アンディ・ウィアー" https://www.amazon.co.jp/exec/obidos/ASIN/4152100702/
81 / * "バッタを倒すぜ　アフリカで (光文社新書 1305) | 前野ウルド浩太郎 |本 | 通販 | Amazon" https://www.amazon.co.jp/dp/4334102905
619 / * "DIE WITH ZERO 人生が豊かになりすぎる究極のルール : ビル・パーキンス, 児島 修: Japanese Books" https://www.amazon.co.jp/dp/4478109680/
```

## 3. (必要なら)蔵書情報のクリア

```
$ CALIL_APPKEY=カーリルのAPIキー cargo run clean
# bookmap.jsonファイルが削除される
```
