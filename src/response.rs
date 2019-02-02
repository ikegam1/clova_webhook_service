use std::error::Error;

//==== Response
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDataStruct{
    version: String,
    #[serde(default)]
    response: serde_json::Value,
    #[serde(rename="sessionAttributes")]
    session_attributes: serde_json::Value
}

pub trait ResponseData{
    fn simple_speech(&self) -> Result<serde_json::Value, Box<dyn Error>>;
    fn set_simple_speech_text(&mut self, s: String, e: bool);
    fn set_reprompt_simple_speech_text(&mut self, s: String);
    fn set_session_attributes_from_str(&mut self, s: String);
    fn set_session_attributes(&mut self, j: serde_json::Value);
    fn new() -> Result<ResponseDataStruct, Box<dyn Error>>;
}

impl ResponseData for ResponseDataStruct {
    fn simple_speech(&self) -> Result<serde_json::Value, Box<dyn Error>>{
        Ok(serde_json::from_str("{ \"key\": \"value\" }").unwrap())
    }
    fn set_simple_speech_text(&mut self, s: String, e: bool){
        self.response["outputSpeech"]["values"]["value"] = serde_json::json!(s);
        self.response["shouldEndSession"] = serde_json::json!(e);
    }
    fn set_reprompt_simple_speech_text(&mut self, s: String){
        self.response["reprompt"] = self.response["outputSpeech"].clone();
        self.response["reprompt"]["outputSpeech"]["values"]["value"] = serde_json::json!(s);
    }
    fn set_session_attributes_from_str(&mut self, s: String){
        self.session_attributes = serde_json::from_str(&s).unwrap();
    }
    fn set_session_attributes(&mut self, j: serde_json::Value){
        self.session_attributes = j;
    }
    fn new() -> Result<ResponseDataStruct, Box<dyn Error>>{
        let r: serde_json::Value = serde_json::from_str(
            r#"{
                "card": {},
                "directives": [],
                "outputSpeech": {
                    "type": "SimpleSpeech",
                    "values": {
                        "lang": "ja",
                        "type": "PlainText",
                        "value": ""
                    }
                },
                "reprompt": {},
                "shouldEndSession": false
            }"#).unwrap();
        let s: serde_json::Value = serde_json::from_str(
            r#"{}"#).unwrap();
        Ok(ResponseDataStruct{ version:"1.0".to_string(), response: r, session_attributes: s })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_test() {
        let mut res: ResponseDataStruct =  ResponseDataStruct::new().unwrap();
        res.set_simple_speech_text("hello world".to_string(), false);
        res.set_reprompt_simple_speech_text("hello world?".to_string());
        res.set_session_attributes_from_str("{\"key\":\"value\"}".to_string());
        assert!(true, format!("{:#?}", res));
    }
}
