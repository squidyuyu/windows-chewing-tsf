// SPDX-License-Identifier: GPL-3.0-or-later
//! 教學標音字型：送字時在破音字後面加「注音 IVS」讀音記號（字嗨 bpmfvs 規格），
//! 讓支援注音 IVS 的字型照使用者打的注音顯示讀音。
//!
//! - 第 k 個讀音（k≥2）加 U+E01E0+(k-1)；第 1 個讀音不加，但本站預設讀音跟規格不同的字
//!   （例：和、倒）加 U+E01EF 鎖定第 1 讀音（教學標音字型擴充；其他 bpmfvs 字型會忽略）。
//! - 「一」「不」交給字型自動變調，不加。
//! - 對照表 ivs_table.rs 由教學標音字型 tools/ime/chewing-ivs/make_ivs_table.py 產生。

use chewing::conversion::Symbol;

#[path = "ivs_table.rs"]
mod ivs_table;
use ivs_table::IVS_TABLE;

const VS_BASE: u32 = 0xE01E0;
const VS_LOCK_FIRST: u32 = 0xE01EF;

/// 按鍵前的組字區：每個字和它的注音（標點等不是注音的位置是 None）
#[derive(Default, Debug, Clone)]
pub(super) struct IvsSnapshot {
    chars: Vec<char>,
    syllables: Vec<Option<String>>,
}

pub(super) fn snapshot(display: &str, symbols: &[Symbol]) -> IvsSnapshot {
    let chars: Vec<char> = display.chars().collect();
    let syllables: Vec<Option<String>> = symbols
        .iter()
        .map(|sym| match sym {
            Symbol::Syllable(syl) => Some(normalize(&syl.to_string())),
            Symbol::Char(_) => None,
        })
        .collect();
    if chars.len() != syllables.len() {
        // 字數和注音數對不上（不應該發生）：不加記號
        return IvsSnapshot::default();
    }
    IvsSnapshot { chars, syllables }
}

/// 新酷音的一聲可能顯示成「ˉ」；對照表的一聲不標
fn normalize(s: &str) -> String {
    s.chars().filter(|&c| c != 'ˉ').collect()
}

fn selector(ch: char, reading: &str) -> Option<char> {
    if ch == '一' || ch == '不' {
        return None;
    }
    let idx = IVS_TABLE.binary_search_by(|(c, _, _)| c.cmp(&ch)).ok()?;
    let (_, lock_first, readings) = IVS_TABLE[idx];
    let k = readings.iter().position(|r| *r == reading)?;
    match k {
        0 if lock_first => char::from_u32(VS_LOCK_FIRST),
        0 => None,
        k if k <= 14 => char::from_u32(VS_BASE + k as u32),
        _ => None,
    }
}

/// 把送出的字串加上讀音記號。送出的字必須是按鍵前組字區的開頭（整段送出或自動送出前半段）。
pub(super) fn annotate(commit: &str, snap: &IvsSnapshot) -> String {
    let commit_chars: Vec<char> = commit.chars().collect();
    if commit_chars.is_empty()
        || commit_chars.len() > snap.chars.len()
        || commit_chars[..] != snap.chars[..commit_chars.len()]
    {
        return commit.to_owned();
    }
    let mut out = String::with_capacity(commit.len() * 2);
    for (i, &ch) in commit_chars.iter().enumerate() {
        out.push(ch);
        if let Some(Some(reading)) = snap.syllables.get(i) {
            if let Some(vs) = selector(ch, reading) {
                out.push(vs);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_sorted() {
        assert!(IVS_TABLE.windows(2).all(|w| w[0].0 < w[1].0));
    }

    #[test]
    fn second_reading_and_lock() {
        let snap = IvsSnapshot {
            chars: "銀行和".chars().collect(),
            syllables: vec![Some("ㄧㄣˊ".into()), Some("ㄏㄤˊ".into()), Some("ㄏㄜˊ".into())],
        };
        let out: Vec<u32> = annotate("銀行和", &snap).chars().map(|c| c as u32).collect();
        assert_eq!(out, vec!['銀' as u32, '行' as u32, 0xE01E1, '和' as u32, 0xE01EF]);
    }

    #[test]
    fn mismatch_is_untouched() {
        let snap = IvsSnapshot::default();
        assert_eq!(annotate("銀行", &snap), "銀行");
    }
}
