use std::io::BufReader;
use std::fs::read;
use std::fmt;
use hex::ToHex;
use std::fs::File;
use std::io::prelude::*;

use log;

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};


pub struct Opcode {
    pub name: String,
    pub hexcode: Vec<u8>,
    pub text_id: Option<u16>
}

impl Opcode {
    pub fn to_hex(&self) -> Vec<u8> {
        self.hexcode.clone()
    }


    pub fn try_from_string(value: String, text_id: u32) -> (Option<Self>, Option<String>) {
        // Brackets and that newline at the end are to be thrown away
        if value.contains("{") || value.contains("}") || value.len() < 2 {
            return (None, None);
        }

        log::debug!("Original: apple {} apple ", value);

        let mut split_string = value.split("(");
        let opcode_text = split_string.next().unwrap();
        log::debug!("opcode_text: '{}'", opcode_text);

        let mut args = split_string.next().unwrap().chars();
        args.next_back(); // Popping off the ending ')'


        let args: Vec<u8> = if opcode_text == "Text" {
            let mut temp: Vec<u8> = Vec::new();
            let _ = temp.write_u16::<LittleEndian>(text_id as u16);

            let mut hexcode: Vec<u8> = Vec::new();
            hexcode.push(112u8);
            hexcode.push(2u8);
            hexcode.append(&mut temp);

            return (Some(Opcode {
                name: "Text".to_string(),
                hexcode: hexcode,
                text_id: None
            }), Some(args.collect()))
        }
        else {
            args
                .as_str()
                .split(",")
                .flat_map(|line| line.trim().parse::<u8>())
                .collect::<Vec<u8>>()
        };

        for arg in args.clone() {
            log::debug!("arg: '{}'", arg);
        }
        
        let opcode: u8 = match opcode_text {
            "0x00"             => 0u8,
            "TextBoxFormat"    => 3u8,
            "Animation"        => 6u8,
            "Voice"            => 8u8,
            "Music"            => 9u8,
            "Sound"            => 10u8,
            "LoadMap"          => 21u8,
            "LoadScript"       => 25u8,
            "StopScript"       => 26u8,
            "Sprite"           => 30u8,
            "ScreenFlash"      => 31u8,
            "SpriteFlash"      => 32u8,
            "Speaker"          => 33u8,
            "ScreenFade"       => 34u8,
            "ChangeUi"         => 37u8,
            "SetFlag"          => 38u8,
            "CheckCharacter"   => 39u8,
            "CheckObject"      => 41u8,
            "SetLabel"         => 42u8,
            "SetChoiceText"    => 43u8,
            "ShowBackground"   => 48u8,
            "0x33"             => 51u8,
            "GoToLabel"        => 52u8,
            "CheckFlagA"       => 53u8,
            "WaitInput"        => 58u8,
            "WaitFrame"        => 59u8,
            "IfFlagCheck"      => 60u8,
            badop        => 254u8
        };

        log::debug!("{}", opcode);

        // if opcode == 999u8 {
        //     return new Error("INVALID OPCODE");
        // }

        let mut hexcode: Vec<u8> = Vec::new();
        hexcode.push(112u8);
        hexcode.push(opcode);
        hexcode.append(&mut args.clone());

        (Some(Opcode {
            name: opcode_text.to_string(),
            hexcode: hexcode,
            text_id: None
        }), None)
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{}(", self.name);

        if self.hexcode.len() == 2 {
            let _ = write!(f, ")");
            return Ok(());
        }

        if self.name == "Text" {
            let _ = write!(f, "{})", self.text_id.unwrap());
            return Ok(());
        }

        for idx in 2..(self.hexcode.len()-1) {
            let _ = write!(f, "{}, ", self.hexcode.get(idx).unwrap());
        }

        let _ = write!(f, "{})", self.hexcode.last().unwrap());
        Ok(())
    }
}
