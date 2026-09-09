use crate::error::{ObError,ObResult};
use serde::{Deserialize,Serialize};
use std::{collections::HashMap,path::Path};

#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct NumericSummary { pub count:u64, pub missing:u64, pub minimum:f64, pub maximum:f64, pub mean:f64 }
#[derive(Debug,Clone,Serialize,Deserialize)] #[serde(rename_all="camelCase")]
pub struct CsvAnalysis { pub path:String, pub row_count:u64, pub column_count:usize, pub headers:Vec<String>, pub numeric_columns:HashMap<String,NumericSummary>, pub parse_warnings:Vec<String> }

pub fn analyze_csv(path:&Path)->ObResult<CsvAnalysis>{
 let mut reader=csv::ReaderBuilder::new().flexible(true).from_path(path).map_err(|error|ObError::Validation(format!("CSV could not be opened: {error}")))?;
 let headers=reader.headers().map_err(|error|ObError::Validation(format!("CSV headers are invalid: {error}")))?.iter().map(str::to_string).collect::<Vec<_>>();
 if headers.is_empty(){return Err(ObError::Validation("CSV has no columns".into()))}
 let mut numbers:Vec<Vec<f64>>=vec![Vec::new();headers.len()]; let mut missing=vec![0u64;headers.len()]; let mut row_count=0u64; let mut warnings=Vec::new();
 for (row_index,result) in reader.records().enumerate(){
  let record=match result{Ok(value)=>value,Err(error)=>{warnings.push(format!("Row {}: {error}",row_index+2));continue}}; row_count+=1;
  for index in 0..headers.len(){let value=record.get(index).unwrap_or("").trim();if value.is_empty(){missing[index]+=1}else if let Ok(number)=value.parse::<f64>(){if number.is_finite(){numbers[index].push(number)}}}
 }
 let mut numeric_columns=HashMap::new();
 for (index,values) in numbers.into_iter().enumerate(){if values.is_empty(){continue}let sum: f64=values.iter().sum();let minimum=values.iter().copied().fold(f64::INFINITY,f64::min);let maximum=values.iter().copied().fold(f64::NEG_INFINITY,f64::max);numeric_columns.insert(headers[index].clone(),NumericSummary{count:values.len() as u64,missing:missing[index],minimum,maximum,mean:sum/values.len() as f64});}
 Ok(CsvAnalysis{path:path.to_string_lossy().into_owned(),row_count,column_count:headers.len(),headers,numeric_columns,parse_warnings:warnings})
}

pub fn as_text(value:&CsvAnalysis)->String{let mut lines=vec![format!("Rows: {}",value.row_count),format!("Columns: {}",value.column_count)];let mut names=value.numeric_columns.keys().cloned().collect::<Vec<_>>();names.sort();for name in names{let item=&value.numeric_columns[&name];lines.push(format!("{name}: count {}, missing {}, min {}, max {}, mean {}",item.count,item.missing,item.minimum,item.maximum,item.mean));}if value.numeric_columns.is_empty(){lines.push("No numeric columns were detected.".into())}if !value.parse_warnings.is_empty(){lines.push(format!("Parse warnings: {}",value.parse_warnings.len()))}lines.join("\n")}

#[cfg(test)] mod tests { use super::*; #[test] fn computes_real_statistics(){let dir=tempfile::tempdir().expect("temp");let path=dir.path().join("data.csv");std::fs::write(&path,"name,value\na,1\nb,3\nc,\n").expect("write");let result=analyze_csv(&path).expect("analysis");assert_eq!(result.row_count,3);assert_eq!(result.numeric_columns["value"].mean,2.0);assert_eq!(result.numeric_columns["value"].missing,1);} }
