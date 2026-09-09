use serde::{Deserialize,Serialize};
#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum ConversationalCue { Frustrated, Excited, Urgent, Sad, Confused, Confident, Curious, Casual }
#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct EmotionalContext { pub cue:ConversationalCue, pub confidence:f32, pub style_instruction:String, pub medical_diagnosis:bool }
pub fn estimate(text:&str)->EmotionalContext{
 let lower=text.to_lowercase(); let score=|terms:&[&str]|terms.iter().filter(|term|lower.contains(**term)).count() as f32;
 let choices=[
  (ConversationalCue::Frustrated,score(&["frustrated","annoying","doesn't work","not working","again!","বিরক্ত","परेशान"]),"Respond calmly, briefly, and lead with the next concrete step."),
  (ConversationalCue::Urgent,score(&["urgent","asap","right now","immediately","জরুরি","तुरंत"]),"Prioritize the action and state blockers immediately."),
  (ConversationalCue::Confused,score(&["confused","don't understand","how does","why is","বুঝতে পারছি না","समझ नहीं"]),"Explain in short sequential steps and verify understanding."),
  (ConversationalCue::Excited,score(&["excited","amazing","great!","চমৎকার","बहुत बढ़िया"]),"Match positive energy without exaggeration."),
  (ConversationalCue::Sad,score(&["sad","upset","disappointed","মন খারাপ","उदास"]),"Use a gentle, respectful tone without making a medical claim."),
  (ConversationalCue::Curious,score(&["curious","wonder","what if","জানতে চাই","जानना"]),"Explore the question clearly and distinguish facts from uncertainty.")
 ];
 let (cue,value,instruction)=choices.into_iter().max_by(|left,right|left.1.partial_cmp(&right.1).unwrap_or(std::cmp::Ordering::Equal)).expect("cue table");
 if value==0.0{EmotionalContext{cue:ConversationalCue::Casual,confidence:.35,style_instruction:"Use a natural, concise, respectful tone.".into(),medical_diagnosis:false}}else{EmotionalContext{cue,confidence:(.55+value*.12).min(.9),style_instruction:instruction.into(),medical_diagnosis:false}}
}
#[cfg(test)] mod tests { use super::*; #[test] fn no_medical_claims(){assert!(!estimate("I am frustrated").medical_diagnosis)} }
