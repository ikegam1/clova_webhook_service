use std::error::Error;

use lambda_http::{lambda, Response, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use clova_webhook_utils::{RequestData, RequestDataStruct, ResponseData, ResponseDataStruct};

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

fn main() -> Result<(), Box<dyn Error>>{
    lambda!(handler);

    Ok(())
}

fn handler(e: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    let req: RequestDataStruct =  serde_json::from_slice(e.body().as_ref()).unwrap();
    let mut speech: String = "hello".to_string();
    let mut end: bool = true;
    match &*req.get_request_type() {
        "LaunchRequest" => {speech = "Hello".to_string(); end=false;},
        "SessionEndedRequest" => {speech = "bye".to_string();},
        _ => {}
    }
    match &*req.get_intent_name() {
        "HelloIntent" => { speech = "hello".to_string(); end=false; },
        "FinishIntent" => { speech = "bye".to_string(); },
        "Clova.GuideIntent" => {speech = "usage..".to_string(); end=false;},
        "Clova.YesIntent" => {speech = "ok.".to_string(); end=false;},
        "Clova.NoIntent" => {speech ="no.".to_string();},
        "Clova.CancelIntent" => {speech = "bye".to_string();},
        "askIntent" => { 
            // String is quoted.. so remove it
            let food: String = &req.get_slots().unwrap().pointer("/foodSlot/value").unwrap().to_string().trim_matches('"').to_string();
    	    if food == "apple".to_string() {
  	        speech = "apple".to_string();
  	    }else{
  	        speech = "pineapple".to_string();
  	    }
        },
        _ => {}
    }
  
    let mut res: ResponseDataStruct =  ResponseDataStruct::new().unwrap();
    res.set_simple_speech_text(speech, end);
    res.set_session_attributes_from_str("{\"current\":\"hello\"}".to_string());
    Ok(
        Response::builder()
           .status(200)
           .header("Content-Type", "application/json; charset=UTF-8")
           .body(
               format!("{}",serde_json::json!(res)).into(),
           )
           .expect("none")
    )
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

    #[test]
    fn response_test() {
        let mut res: ResponseDataStruct =  ResponseDataStruct::new().unwrap();
        res.set_simple_speech_text("hello world".to_string(), false);
        res.set_reprompt_simple_speech_text("hello world?".to_string());
        res.set_session_attributes_from_str("{\"key\":\"value\"}".to_string());
        assert!(true, format!("{:#?}", res));
    }
}
