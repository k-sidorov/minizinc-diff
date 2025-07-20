// minizinc-diff
// Copyright (C) 2025 Konstantin Sidorov
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// This trait is imported from https://github.com/TianyiShi2001/hhmmss/blob/main/src/lib.rs
pub trait Hhmmss {
    fn sms(&self) -> (i64, i64);
    fn _hhmmss(&self) -> String {
        let (s, _) = self.sms();
        _s2hhmmss(s)
    }
    fn hhmmssxxx(&self) -> String {
        let (s, ms) = self.sms();
        sms2hhmmsxxx(s, ms)
    }
}

impl Hhmmss for std::time::Duration {
    fn sms(&self) -> (i64, i64) {
        let s = self.as_secs();
        let ms = self.subsec_millis();
        (s as i64, ms as i64)
    }
}

fn _s2hhmmss(s: i64) -> String {
    let mut neg = false;
    let mut s = s;
    if s < 0 {
        neg = true;
        s = -s;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!("{}{:02}:{:02}:{:02}", if neg { "-" } else { "" }, h, m, s)
}

fn sms2hhmmsxxx(s: i64, ms: i64) -> String {
    let mut neg = false;
    let (mut s, mut ms) = (s, ms);
    if s < 0 {
        neg = true;
        s = -s;
        ms = -ms;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!(
        "{}{:02}:{:02}:{:02}.{:03}",
        if neg { "-" } else { "" },
        h,
        m,
        s,
        ms
    )
}
