use std::f32::consts::PI;
use std::io::{Write, Seek};
use hound::WavWriter;

// 定数
const SAMPLE_RATE: u32 = 44100;
const BPM: f32 = 120.0;

#[allow(unused_variables)]
fn main() {
  // WavWriterオブジェクトを生成
  let spec = houd::WavSpec {
    chanels: 1,
    sample_rate: SAMPLE_RATE as u32,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };
  let mut fw = WavWriter::create("melody.wav", spec).unwrap();
  // 音階を定義
  let (c4, d4, e4, f4) = (261.626, 293.665, 329.628, 349.228);
  let (g4, a4, b4, c5) = (391.995, 440.000, 493.883, 523.251);
  // 音長を定義
  let l4 = ((60.0 / bpm) * SAMPLE_RATE) as u32;
  let l2 = l4 * 2;
  // メロディを指定
  write_tone(&mut fw, c4, l4);
}

// 指定の音階を指定の長さ分書き込む
// fn wite_tone<w>

