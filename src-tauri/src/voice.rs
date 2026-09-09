use crate::error::{ObError,ObResult};
use serde::{Deserialize,Serialize};
use std::{path::Path,process::Command};

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct VoiceEngineStatus { pub id:String, pub label:String, pub kind:String, pub available:bool, pub languages:String, pub detail:String }
pub fn detect()->Vec<VoiceEngineStatus>{vec![
 status("piper","Piper","tts",which::which("piper").is_ok(),"Model dependent"),
 status("windows-sapi","Windows SAPI","tts",cfg!(target_os="windows"),"Installed Windows voices"),
 status("coqui","Coqui-compatible TTS","tts",which::which("tts").is_ok(),"Model dependent"),
 status("kokoro","Kokoro-compatible TTS","tts",which::which("kokoro-tts").is_ok(),"Model dependent"),
 status("espeak-ng","eSpeak NG","tts",which::which("espeak-ng").is_ok(),"Engine dependent"),
 status("whisper","Whisper","stt",which::which("whisper").is_ok(),"Model dependent"),
 status("faster-whisper","faster-whisper","stt",which::which("faster-whisper").is_ok(),"Model dependent"),
 status("vosk","Vosk-compatible","stt",which::which("vosk-transcriber").is_ok(),"Model dependent")
]}
fn status(id:&str,label:&str,kind:&str,available:bool,languages:&str)->VoiceEngineStatus{VoiceEngineStatus{id:id.into(),label:label.into(),kind:kind.into(),available,languages:languages.into(),detail:if available{"AVAILABLE".into()}else{"REQUIRES CONFIGURATION".into()}}}
pub fn speak(engine:&str,text:&str,voice:Option<&str>)->ObResult<()>{
 if text.trim().is_empty(){return Err(ObError::Validation("Speech text is empty".into()))}
 let status=detect().into_iter().find(|item|item.id==engine).ok_or_else(||ObError::Validation("Unknown voice engine".into()))?;
 if !status.available{return Err(ObError::RequiresConfiguration(format!("{} is not installed",status.label)))}
 let result=match engine{
  "windows-sapi"=>{let escaped=text.replace('\'',"''");let select=voice.map(|value|format!("$s.SelectVoice('{}');",value.replace('\'',"''"))).unwrap_or_default();let script=format!("Add-Type -AssemblyName System.Speech; $s=New-Object System.Speech.Synthesis.SpeechSynthesizer; {select} $s.Speak('{escaped}')");Command::new("powershell.exe").args(["-NoProfile","-NonInteractive","-Command",&script]).status()},
  "espeak-ng"=>Command::new("espeak-ng").arg(text).status(),
  _=>return Err(ObError::RequiresConfiguration("This engine requires a model path configured in Voice settings".into()))
 }?;
 if !result.success(){return Err(ObError::Unavailable("Voice engine returned a failure status".into()))}Ok(())
}
pub fn transcribe(engine:&str,audio:&Path,language:Option<&str>)->ObResult<String>{if !audio.exists(){return Err(ObError::Validation("Audio file does not exist".into()))}let executable=match engine{"whisper"=>"whisper","faster-whisper"=>"faster-whisper","vosk"=>"vosk-transcriber",_=>return Err(ObError::Validation("Unknown speech recognition engine".into()))};if which::which(executable).is_err(){return Err(ObError::RequiresConfiguration(format!("{executable} is not installed")))}let mut command=Command::new(executable);command.arg(audio);if let Some(value)=language{command.args(["--language",value]);}let output=command.output()?;if !output.status.success(){return Err(ObError::Unavailable(String::from_utf8_lossy(&output.stderr).into_owned()))}Ok(String::from_utf8_lossy(&output.stdout).into_owned())}
