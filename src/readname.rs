use encoding_rs::SHIFT_JIS;
use regex::Regex;
use std::io;
use std::path::Path;
use std::process::Command;

pub trait Fromcard {
    fn readcard() -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
#[derive(Debug, Clone)]
pub struct User {
    userid: String,
    username: String,
}

impl Fromcard for User {
    fn readcard() -> Result<Self, Box<dyn std::error::Error>> {
        let output = Command::new("./felica_dump")
            .current_dir(Path::new("libpafe/tests"))
            .output()?;
        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            let vec = Self::extract_bytes(&stdout)?;
            let name = Self::decode_shift_jis(&vec);
            let id = Self::get_cardid(&stdout)?;
            Ok(Self {
                userid: id,
                username: name,
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Command failed: {}", stderr).into())
        }
    }
}

impl User {
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
    pub fn get_userid(&self) -> String {
        self.userid.clone()
    }
    fn extract_bytes(dump: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let re = Regex::new(r"006A:0001:([0-9A-F]{32})")?;
        if let Some(cap) = re.captures(dump) {
            let hex_str = &cap[1];
            let bytes = (0..hex_str.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16))
                .collect::<Result<Vec<u8>, _>>()?;
            Ok(bytes)
        } else {
            Err("Target line not found".into())
        }
    }
    fn decode_shift_jis(bytes: &[u8]) -> String {
        let (cow, _, _) = SHIFT_JIS.decode(bytes);
        cow.to_string()
            .replace("\u{0000}", "")
            .replace("\u{ff9e}", "゛")
            .replace("\u{ff9e}", "゜")
    }
    fn get_cardid(dump: &str) -> Result<String, Box<dyn std::error::Error>> {
        let re = Regex::new(r"card IDm = ([0-9A-F]{16})")?;
        if let Some(cap) = re.captures(dump) {
            Ok(cap[1].to_string())
        } else {
            Err("Not found!".into())
        }
    }
    pub fn get_userinfo() -> Self {
        println!(
            "Hello! RCC図書館へようこそ．\nカードリーダーに学生証を置いたらEnterを押してください．"
        );

        for attempt in 1..=5 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            println!("読み込み中... (試行 {}/{})", attempt, 5);

            match Self::readcard() {
                Ok(user) => return user,
                Err(e) => {
                    eprintln!("カードの読み取りに失敗しました：{}", e);
                    if attempt == 5 {
                        eprintln!("5回連続で失敗したため、終了します。");
                        std::process::exit(1);
                    } else {
                        println!("もう一度カードを置いて Enter を押してください。");
                    }
                }
            }
        }

        unreachable!();
    }
}
