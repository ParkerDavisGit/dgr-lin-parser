use std::io::BufReader;
use std::fs::read;
use std::fmt;
use hex::ToHex;
use std::fs::File;
use std::io::prelude::*;

// test.bin
// 70 00 BF 01 
// 70 22 01 00 01 
// 70 25 0B 01 
// 70 25 07 00 
// 70 25 10 00 
// 70 33 02 00 00 64 
// 70 33 03 00 00 64
//


// Operation    
// output -> 70 02 a1 a2 .... an
//
// ParsedCommand
// output -> command: a1, a2, ..., an

pub enum Op {
    Zero {
        arg1: u8,
        arg2: u8,
    },
    ScreenFade {
        arg1: u8,
        arg2: u8,
        arg3: u8,
    },
    ChangeUi {
        arg1: u8,
        arg2: u8,
    },
    ThreeThree {
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
    },
    SetFlag {
        arg1: u8,
        arg2: u8,
        arg3: u8,
    },
    LoadMap {
        arg1: u8, // Map ID
        arg2: u8,
        arg3: u8,
    },
    Sprite {
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
        arg5: u8,
    },
    Music {
        arg1: u8,
        arg2: u8,
        arg3: u8,
    },
    ShowBackground {
        arg1: u8,
        arg2: u8,
        arg3: u8,
    },
    Speaker {
        arg1: u8, // Speaker ID
    },
    TextBoxFormat { // 03
        arg1: u8, // Speaker ID
    },
    Text { // 02
        arg1: u8, // Speaker ID
        arg2: u8, // Speaker ID
    },
    WaitFrame,
    WaitInput,
    Sound { // 0A
        arg1: u8,
        arg2: u8,
        arg3: u8,
    }, 
    ScreenFlash {// 1F
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
        arg5: u8,
        arg6: u8,
        arg7: u8,
    },
    Animation {// 06
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
        arg5: u8,
        arg6: u8,
        arg7: u8,
        arg8: u8,
    },
    Voice { // 08
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
        arg5: u8,
    },
    SpriteFlash { // 20
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
        arg5: u8,
    },
    GoToLabel {// 34
        arg1: u8,
        arg2: u8,
    },
    CheckCharacter {
        arg1: u8,
    },
    CheckObject {
        arg1: u8,
    },
    CheckFlagA4 { // 35
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
    },
    CheckFlagA24 { // 35
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
        arg5: u8,
        arg6: u8,
        arg7: u8,
        arg8: u8,
        arg9: u8,
        arg10: u8,
        arg11: u8,
        arg12: u8,
        arg13: u8,
        arg14: u8,
        arg15: u8,
        arg16: u8,
        arg17: u8,
        arg18: u8,
        arg19: u8,
        arg20: u8,
        arg21: u8,
        arg22: u8,
        arg23: u8,
        arg24: u8,
    },
    CheckFlagA19 { // 35
        arg1: u8,
        arg2: u8,
        arg3: u8,
        arg4: u8,
        arg5: u8,
        arg6: u8,
        arg7: u8,
        arg8: u8,
        arg9: u8,
        arg10: u8,
        arg11: u8,
        arg12: u8,
        arg13: u8,
        arg14: u8,
        arg15: u8,
        arg16: u8,
        arg17: u8,
        arg18: u8,
        arg19: u8,
    },
    If_FlagCheck, //3C
    SetLabel { // 2A
        arg1: u8,
        arg2: u8,
    },
    SetChoiceText {
        arg1: u8,
    },
    LoadScript {
        arg1: u8,
        arg2: u8,
        arg3: u8,
    },
    StopScript,
}

impl Op {
    fn to_hex(&self) -> Vec<u8> {
        match self {
            Op::Zero { arg1, arg2 } => 
                vec![112u8, 0u8, *arg1, *arg2],
            Op::Text { arg1, arg2 } => 
                vec![112u8, 2u8, *arg1, *arg2],
            Op::TextBoxFormat { arg1 } => 
                vec![112u8, 3u8, *arg1],
            Op::Animation { arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8 } => 
                vec![112u8, 6u8, *arg1, *arg2, *arg3, *arg4, *arg5, *arg6, *arg7, *arg8],
            Op::Voice { arg1, arg2, arg3, arg4, arg5 } => 
                vec![112u8, 8u8, *arg1, *arg2, *arg3, *arg4, *arg5],
            Op::Music { arg1, arg2, arg3 } => 
                vec![112u8, 9u8, *arg1, *arg2, *arg3],
            Op::Sound { arg1, arg2, arg3 } => 
                vec![112u8, 10u8, *arg1, *arg2, *arg3],
            Op::LoadMap { arg1, arg2, arg3 } =>
                vec![112u8, 21u8, *arg1, *arg2, *arg3],
            Op::LoadScript { arg1, arg2, arg3 } => 
                vec![112u8, 25u8, *arg1, *arg2, *arg3],
            Op::StopScript => 
                vec![112u8, 26u8],
            Op::Sprite { arg1, arg2, arg3, arg4, arg5 } => 
                vec![112u8, 30u8, *arg1, *arg2, *arg3, *arg4, *arg5],
            Op::ScreenFlash { arg1, arg2, arg3, arg4, arg5, arg6, arg7 } => 
                vec![112u8, 31u8, *arg1, *arg2, *arg3, *arg4, *arg5, *arg6, *arg7],
            Op::SpriteFlash { arg1, arg2, arg3, arg4, arg5 } => 
                vec![112u8, 32u8, *arg1, *arg2, *arg3, *arg4, *arg5],
            Op::Speaker { arg1 } => 
                vec![112u8, 33u8, *arg1],
            Op::ScreenFade { arg1, arg2, arg3 } => 
                vec![112u8, 34u8, *arg1, *arg2, *arg3],
            Op::ChangeUi { arg1, arg2 } => 
                vec![112u8, 37u8, *arg1, *arg2],
            Op::SetFlag { arg1, arg2, arg3 } => 
                vec![112u8, 38u8, *arg1, *arg2, *arg3],
            Op::CheckCharacter { arg1 } => 
                vec![112u8, 39u8, *arg1],
            Op::CheckObject { arg1 } => 
                vec![112u8, 41u8, *arg1],
            Op::SetLabel { arg1, arg2 } => 
                vec![112u8, 42u8, *arg1, *arg2],
            Op::SetChoiceText { arg1 } => 
                vec![112u8, 43u8, *arg1],
            Op::ShowBackground { arg1, arg2, arg3 } => 
                vec![112u8, 48u8, *arg1, *arg2, *arg3],
            Op::ThreeThree { arg1, arg2, arg3, arg4 } => 
                vec![112u8, 51u8, *arg1, *arg2, *arg3, *arg4],
            Op::GoToLabel { arg1, arg2 } => 
                vec![112u8, 52u8, *arg1, *arg2],
            Op::CheckFlagA4 { arg1, arg2, arg3, arg4 } => 
                vec![112u8, 53u8, *arg1, *arg2, *arg3, *arg4],
            Op::CheckFlagA24 { arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24 } => 
                vec![112u8, 53u8, *arg1, *arg2, *arg3, *arg4, *arg5, *arg6, *arg7, *arg8, *arg9, *arg10, *arg11, *arg12, *arg13, *arg14, *arg15, *arg16, *arg17, *arg18, *arg19, *arg20, *arg21, *arg22, *arg23, *arg24],
            Op::CheckFlagA19 { arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19} => 
                vec![112u8, 53u8, *arg1, *arg2, *arg3, *arg4, *arg5, *arg6, *arg7, *arg8, *arg9, *arg10, *arg11, *arg12, *arg13, *arg14, *arg15, *arg16, *arg17, *arg18, *arg19],
            Op::WaitInput =>
                vec![112u8, 58u8],
            Op::WaitFrame =>
                vec![112u8, 59u8],
            Op::If_FlagCheck =>
                vec![112u8, 60u8],
        }
    }   
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Zero { arg1, arg2 } => write!(f, "0x00({}, {})", arg1, arg2),
            Op::ScreenFade { arg1, arg2, arg3 } => write!(f, "ScreenFade({}, {}, {})", arg1, arg2, arg3),
            Op::ChangeUi { arg1, arg2 } => write!(f, "ChangeUi({}, {})", arg1, arg2),
            Op::ThreeThree { arg1, arg2, arg3, arg4 } => write!(f, "0x33({}, {}, {}, {})", arg1, arg2, arg3, arg4),
            Op::SetFlag { arg1, arg2, arg3 } => write!(f, "SetFlag({}, {}, {})", arg1, arg2, arg3),
            Op::LoadMap { arg1, arg2, arg3 } =>write!(f, "LoadMap({}, {}, {})", arg1, arg2, arg3), 
            Op::Sprite { arg1, arg2, arg3, arg4, arg5 } => write!(f, "Sprite({}, {}, {}, {}, {})", arg1, arg2, arg3, arg4, arg5),
            Op::Music { arg1, arg2, arg3 } => write!(f, "Music({}, {}, {})", arg1, arg2, arg3),
            Op::ShowBackground { arg1, arg2, arg3 } => write!(f, "ShowBackground({}, {}, {})", arg1, arg2, arg3),
            Op::Speaker { arg1 } => write!(f, "Speaker({})", arg1),
            Op::TextBoxFormat { arg1 } => write!(f, "TextBoxFormat({})", arg1),
            Op::Text { arg1, arg2 } => write!(f, "Text({}, {})", *arg1, *arg2),
            Op::WaitFrame =>write!(f, "WaitFrame()"),
            Op::WaitInput =>write!(f, "WaitInput()"),
            Op::Sound { arg1, arg2, arg3 } => write!(f, "Sound({}, {}, {})", arg1, arg2, arg3),
            Op::ScreenFlash { arg1, arg2, arg3, arg4, arg5, arg6, arg7 } => write!(f, "SpriteFlash({}, {}, {}, {}, {}, {}, {})", arg1, arg2, arg3, arg4, arg5, arg6, arg7),
            Op::Animation { arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8 } => write!(f, "SpriteFlash({}, {}, {}, {}, {}, {}, {}, {})", arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8),
            Op::Voice { arg1, arg2, arg3, arg4, arg5 } => write!(f, "Voice({}, {}, {}, {}, {})", arg1, arg2, arg3, arg4, arg5),
            Op::SpriteFlash { arg1, arg2, arg3, arg4, arg5 } => write!(f, "SpriteFlash({}, {}, {}, {}, {})", arg1, arg2, arg3, arg4, arg5),
            Op::GoToLabel { arg1, arg2 } => write!(f, "GoToLabel({}, {})", arg1, arg2),
            Op::CheckCharacter { arg1 } => write!(f, "CheckCharacter({})", arg1),
            Op::CheckObject { arg1 } => write!(f, "CheckObject({})", arg1),
            Op::CheckFlagA4 { arg1, arg2, arg3, arg4 } => write!(f, "CheckFlagA4({}, {}, {}, {})", arg1, arg2, arg3, arg4),
            Op::CheckFlagA24 { arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24 } => write!(f, "CheckFlagA24({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24),
            Op::CheckFlagA19 { arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19} => write!(f, "CheckFlagA19({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19),
            Op::If_FlagCheck =>write!(f, "If_FlagCheck()"),
            Op::SetLabel { arg1, arg2 } => write!(f, "SetLabel({}, {})", arg1, arg2),
            Op::SetChoiceText { arg1 } => write!(f, "SetChoiceText({})", arg1),
            Op::LoadScript { arg1, arg2, arg3 } => write!(f, "LoadScript({}, {}, {})", arg1, arg2, arg3),
            Op::StopScript => write!(f, "StopScript()"),
        }
    }
}


impl TryFrom<String> for Op {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split_string = value.split("(");
        let opcode = split_string.next().unwrap();

        let mut args = split_string.next().unwrap().chars();
        args.next_back();

        let args: Vec<u8> = args
            .as_str()
            .split(",")
            .flat_map(|line| line.trim().parse::<u8>())
            .collect();

        match opcode {
            "0x00" => Ok(Op::Zero {arg1: args[0], arg2: args[1]}),
            "ScreenFade" => Ok(Op::ScreenFade {arg1: args[0], arg2: args[1], arg3: args[2]}),
            "ChangeUi" => Ok(Op::ChangeUi {arg1: args[0], arg2: args[1]}),
            "0x33" => Ok(Op::ThreeThree {arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3]}),
            "SetFlag" => Ok(Op::SetFlag {arg1: args[0], arg2: args[1], arg3: args[2]}),
            "LoadMap" => Ok(Op::LoadMap {arg1: args[0], arg2: args[1], arg3: args[2]}),
            "Sprite" => Ok(Op::Sprite {arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], arg5: args[4]}),
            "Music" => Ok(Op::Music {arg1: args[0], arg2: args[1], arg3: args[2]}),
            "ShowBackground" => Ok(Op::ShowBackground {arg1: args[0], arg2: args[1], arg3: args[2]}),
            "Speaker" => Ok(Op::Speaker {arg1: args[0]}),
            "TextBoxFormat" => Ok(Op::TextBoxFormat {arg1: args[0]}),
            "Text" => Ok(Op::Text {arg1: args[0], arg2: args[1]}),
            "WaitFrame" => Ok(Op::WaitFrame),
            "WaitInput" => Ok(Op::WaitInput),
            "Sound" => Ok(Op::Sound {arg1: args[0], arg2: args[1], arg3: args[2]}),
            "ScreenFlash" => Ok(Op::ScreenFlash {arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], arg5: args[4], arg6: args[5], arg7: args[6]}),
            "Animation" => Ok(Op::Animation {arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], arg5: args[4], arg6: args[5], arg7: args[6], arg8: args[7]}),
            "Voice" => Ok(Op::Voice {arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], arg5: args[4]}),
            "SpriteFlash" => Ok(Op::SpriteFlash {arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], arg5: args[4]}),
            "GoToLabel" => Ok(Op::GoToLabel {arg1: args[0], arg2: args[1]}),
            "CheckCharacter" => Ok(Op::CheckCharacter {arg1: args[0]}),
            "CheckObject" => Ok(Op::CheckObject {arg1: args[0]}),
            "CheckFlagA4" => Ok(Op::CheckFlagA4 {arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3]}),
            "CheckFlagA24" => Ok(Op::CheckFlagA24 {
                    arg1: args[0], 
                    arg2: args[1], 
                    arg3: args[2], 
                    arg4: args[3], 
                    arg5: args[4], 
                    arg6: args[5], 
                    arg7: args[6], 
                    arg8: args[7],
                    arg9: args[8], 
                    arg10: args[9], 
                    arg11: args[10],
                    arg12: args[11], 
                    arg13: args[12],
                    arg14: args[13], 
                    arg15: args[14],
                    arg16: args[15], 
                    arg17: args[16],
                    arg18: args[17], 
                    arg19: args[18],
                    arg20: args[19], 
                    arg21: args[20],
                    arg22: args[21], 
                    arg23: args[22],
                    arg24: args[23],
                }),
            "CheckFlagA19" => Ok(Op::CheckFlagA19 {
                arg1: args[0], 
                arg2: args[1], 
                arg3: args[2], 
                arg4: args[3], 
                arg5: args[4], 
                arg6: args[5], 
                arg7: args[6], 
                arg8: args[7],
                arg9: args[8], 
                arg10: args[9], 
                arg11: args[10],
                arg12: args[11], 
                arg13: args[12],
                arg14: args[13], 
                arg15: args[14],
                arg16: args[15], 
                arg17: args[16],
                arg18: args[17], 
                arg19: args[18],
            }),
            "If_FlagCheck" => Ok(Op::If_FlagCheck),
            "SetLabel" => Ok(Op::SetLabel {arg1: args[0], arg2: args[1]}),
            "SetChoiceText" => Ok(Op::SetChoiceText {arg1: args[0]}),
            "LoadScript" => Ok(Op::LoadScript {arg1: args[0], arg2: args[1], arg3: args[2]}),
            "StopScript" => Ok(Op::StopScript),
            badop => {
                println!("Invalid Opcode: {}", badop);
                Err("Invalid Opcode")
            },
        }
    }
}


fn main() -> Result<(), eyre::Report> {
    //let data = read("data/e00_004_003.bytecode").unwrap();
    
    byte_to_text();
    //text_to_byte();

    Ok(())
}


fn text_to_byte() -> Result<(), eyre::Report> {
    let f = File::open("data/output.txt")?;
    let mut reader = BufReader::new(f).lines().flatten();

    let mut bytes: Vec<u8> = Vec::new();

    for line in reader {
        bytes.append(&mut Op::try_from(line).unwrap().to_hex());
    }

    println!("Parsed!");
    //let ops: Vec<Result<Op, &'static str>> = ops.into_iter().flatten().collect();

    let mut file = File::create("output.bin")?;

    file.write(&bytes[..]);
    
    println!("Writen!");

    Ok(())
}


fn byte_to_text() -> Result<(), eyre::Report> {
    //let mut data = read("data/e00_004_003.bytecode").unwrap().into_iter().peekable();
    let mut data = read("data/new_output.bin").unwrap().into_iter().peekable();
    let mut ops: Vec<Op> = Vec::new();

    loop {
        if data.peek() == None {
            break;
        }
        if data.next().unwrap() != 112u8 {
            break;
        }

        let cmd = data.next();
        match vec![cmd.unwrap()].encode_hex::<String>().as_str() {
            "00" => {
                ops.push(Op::Zero {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(),
                })
            }
            "02" => {
                ops.push(Op::Text { // 02
                    arg1: data.next().unwrap(), // Text ID
                    arg2: data.next().unwrap(), // Text ID 2
                })
            }
            "03" => {
                ops.push(Op::TextBoxFormat { // 03
                    arg1: data.next().unwrap(), // Speaker ID
                })
            }
            "06" => {
                ops.push(Op::Animation {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                    arg6: data.next().unwrap(), 
                    arg7: data.next().unwrap(), 
                    arg8: data.next().unwrap(), 
                })
            }
            "08" => {
                ops.push(Op::Voice { // 08
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                })
            }
            "09" => {
                ops.push(Op::Music {
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "0a" => {
                ops.push(Op::Sound { // 0A
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "15" => {
                ops.push(Op::LoadMap {
                    arg1: data.next().unwrap(),  // Map ID
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                })
            }
            "19" => {
                ops.push(Op::LoadScript {
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                })
            }
            "1a" => {
                ops.push(Op::StopScript)
            }
            "1e" => {
                ops.push(Op::Sprite { // 1E
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                })
            }
            "1f" => {
                ops.push(Op::ScreenFlash {// 1F
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                    arg6: data.next().unwrap(), 
                    arg7: data.next().unwrap(), 
                })
            }
            "20" => {
                ops.push(Op::SpriteFlash { // 20
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(), 
                    arg3: data.next().unwrap(), 
                    arg4: data.next().unwrap(), 
                    arg5: data.next().unwrap(), 
                })
            }
            "21" => {
                ops.push(Op::Speaker {
                    arg1: data.next().unwrap(),  // Speaker ID
                })
            }
            "22" => {
                ops.push(Op::ScreenFade {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "25" => {
                ops.push(Op::ChangeUi {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(),
                })
            }
            "26" => {
                ops.push(Op::SetFlag {
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "27" => {
                ops.push(Op::CheckCharacter {
                    arg1: data.next().unwrap(),
                })
            }
            "29" => {
                ops.push(Op::CheckObject {
                    arg1: data.next().unwrap(),
                })
            }
            "2a" => {
                ops.push(Op::SetLabel { // 2A
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                })
            }
            "2b" => {
                ops.push(Op::SetChoiceText {
                    arg1: data.next().unwrap(),
                })
            }
            "30" => {
                ops.push(Op::ShowBackground {
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                })
            }
            "33" => {
                ops.push(Op::ThreeThree {
                    arg1: data.next().unwrap(), 
                    arg2: data.next().unwrap(),
                    arg3: data.next().unwrap(),
                    arg4: data.next().unwrap(),
                })
            }
            "34" => {
                ops.push(Op::GoToLabel {// 34
                    arg1: data.next().unwrap(),
                    arg2: data.next().unwrap(),
                })
            }
            "35" => {
                let mut args: Vec<u8> = Vec::new();
                let mut idx = 0u8;

                while data.peek().unwrap() != &112u8 {
                    args.push(data.next().unwrap());
                    idx = idx + 1u8;
                }

                match args.len() {
                    4 => {
                        ops.push(Op::CheckFlagA4 { // 35
                            arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3],
                        });
                    }

                    19 => {
                        ops.push(Op::CheckFlagA19 { // 35
                            arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], 
                            arg5: args[4], arg6: args[5], arg7: args[6], arg8: args[7], 
                            arg9: args[8], arg10: args[9], arg11: args[10], arg12: args[11], 
                            arg13: args[12], arg14: args[13], arg15: args[14], arg16: args[15], 
                            arg17: args[16], arg18: args[17], arg19: args[18],
                        });
                    }

                    24 => {
                        ops.push(Op::CheckFlagA24 { // 35
                            arg1: args[0], arg2: args[1], arg3: args[2], arg4: args[3], 
                            arg5: args[4], arg6: args[5], arg7: args[6], arg8: args[7], 
                            arg9: args[8], arg10: args[9], arg11: args[10], arg12: args[11], 
                            arg13: args[12], arg14: args[13], arg15: args[14], arg16: args[15], 
                            arg17: args[16], arg18: args[17], arg19: args[18], arg20: args[19],
                            arg21: args[20], arg22: args[21], arg23: args[22], arg24: args[23],
                        });
                    }

                    length => {
                        eyre::bail!("Invalid CheckFlagA Length: {}", length);
                    }
                }
            }
            "3a" => {
                ops.push(Op::WaitInput)
            }
            "3b" => {
                ops.push(Op::WaitFrame)
            }
            "3c" => {
                ops.push(Op::If_FlagCheck)
            }
            default => {
                for line in ops {
                    println!("{}", line);
                }
                eyre::bail!("Invalid opcode: {}", default);
            }
        }
    }
    println!("Parsed!");
    //let ops: Vec<Result<Op, &'static str>> = ops.into_iter().flatten().collect();

    let mut file = File::create("output.txt")?;

    for line in ops {
        write!(file, "{}\n", line);
    }
    
    println!("Writen!");

    Ok(())
}