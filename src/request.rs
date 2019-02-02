//==== Request
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDataStruct {
    version: String,
    #[serde(default)]
    session: serde_json::Value,
    #[serde(default)]
    context: serde_json::Value,
    #[serde(default)]
    request: serde_json::Value
}

pub trait RequestData {
    fn get_session(&self) -> Result<&serde_json::Value, String>;
    fn get_context(&self) -> Result<&serde_json::Value, String>;
    fn get_request(&self) -> Result<&serde_json::Value, String>;
    fn get_request_type(&self) -> String;
    fn get_intent_name(&self) -> String;
    fn get_slots(&self) -> Result<&serde_json::Value, String>;
    fn get_session_attributes(&self) -> Result<&serde_json::Value, String>;
}

impl RequestData for RequestDataStruct {
    fn get_session(&self) -> Result<&serde_json::Value, String>{
        match &self.session.is_object(){
            false => Err("session is empty".to_string()),
            _ => Ok(&self.session)
        }
    }
    fn get_context(&self) -> Result<&serde_json::Value, String>{
        match &self.context.is_object(){
            false => Err("context is empty".to_string()),
            _ => Ok(&self.context)
        }
    }
    fn get_request(&self) -> Result<&serde_json::Value, String>{
        match &self.request.is_object(){
            false => Err("request is empty".to_string()),
            _ => Ok(&self.request)
        }
    }
    fn get_request_type(&self) -> String{
        let e = "\"unknown type\"";
        let s = serde_json::to_string(&self.request.get("type")).unwrap_or(e);
        let mut ar = s.split("\"");
        ar.next().expect(e);
        ar.next().expect(e).to_string()
    }
    fn get_intent_name(&self) -> String{
        let e = "\"unknown intent name\"";
        let s = serde_json::to_string(&self.request["intent"]["name"]).unwrap_or(e);
        let mut ar = s.split("\"");
        ar.next().expect(e);
        ar.next().expect(e).to_string()
    }
    fn get_slots(&self) -> Result<&serde_json::Value, String>{
        match &self.request.pointer("/intent/slots"){
            None => Err("slots is empty".to_string()),
            _ => Ok(&self.request["intent"]["slots"])
        }
    }

    fn get_session_attributes(&self) -> Result<&serde_json::Value, String>{
        match &self.session.pointer("/sessionAttributes"){
            None => Err("sessionAttributes is empty".to_string()),
            _ => Ok(&self.session["sessionAttributes"])
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_test() {
        let data = r#"
	{
	    "version": "1.0",
	    "session": {
		"sessionId": "xxxxxxxx",
		"sessionAttributes": {"a": 1 },
		"user": {
		    "userId": "xxxxxxxx",
		    "accessToken": "xxxxxxx"
		},
		"new": true
	    },
	    "context": {
		"System": {
		    "application": {
			"applicationId": "xxxxxxxx"
		    },
		    "user": {
			"userId": "xxxxxxxx",
			"accessToken": "xxxxxxxx"
		    },
		    "device": {
			"deviceId": "xxxxxxxx",
			"display": {
			    "size": "l100",
			    "orientation": "landscape",
			    "dpi": 96,
			    "contentLayer": {
				"width": 640,
				"height": 360
			    }
			}
		    }
		}
	    },
	    "request": {
		"type": "IntentRequest",
		"intent": {
		    "name": "helloIntent",
		    "slots": {
			"hello": {
			    "name": "hello",
			    "value": "Hello World"
			}
		    }
		}
	    }
	}
	"#;
        let req: RequestDataStruct = serde_json::from_str(data).unwrap();
        assert_eq!("IntentRequest", req.get_request_type());
        assert_eq!("helloIntent", req.get_intent_name());
        assert_eq!("hello", req.get_slots().unwrap().pointer("/hello/name").unwrap());
        assert_eq!(1, req.get_session_attributes().unwrap().pointer("/a").unwrap().as_i64().unwrap());
    }
}
