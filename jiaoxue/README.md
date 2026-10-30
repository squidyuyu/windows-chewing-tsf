# 新酷音（教學標音版）

以新酷音 TSF v26.7.2.0 為底，加了兩件事（教學標音字型 https://github.com/squidyuyu/-jiaoxue-pinyin-font 的輸入法實驗）：

1. **送字時加注音 IVS 讀音記號**（tip/src/text_service/ivs.rs）：破音字後面依使用者打的注音加 U+E01E1…（字嗨 bpmfvs 規格），
   第 1 讀音在教學標音字型預設不同的字加 U+E01EF 鎖定。用支援注音 IVS 的字型（教學標音字型 v4.7 以後、字嗨注音字型）就會照打的注音顯示。
2. **加強詞庫**（jiaoxue/tsi.csv、word.csv）：新酷音詞庫「只加不刪」，加上台灣國小課本讀音（讀音容錯）與補詞；CI 建置時重編 tsi.dat、word.dat。

授權：程式 GPL-3.0-or-later（同新酷音 TSF）；詞庫 LGPL-2.1-or-later（新酷音詞庫）＋教育部辭典讀音（CC BY-ND，散布前要確認）。
