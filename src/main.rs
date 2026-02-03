use tokio::{self, net::TcpListener};
use log;
use std::fs;
use std::str::FromStr;
use std::{
    fs::File, 
    io::Write,
    path::Path,
    env
};
use bytes::Bytes;
use env_logger;
use axum::{
    Router, 
    body::Body, 
    http::{
        StatusCode, 
        header::HeaderMap
    }, 
    routing::{
        post
    },
    extract::{
        State,
        Json
    },
    response::{
        IntoResponse,
        Response
    }
};
use std::sync::{
    Arc,
    Mutex
};
pub mod campaign;
pub mod player;
pub mod pingVerifier;
use pingVerifier::PingVerifier;

pub mod discord_data_structs;
use discord_data_structs::Interaction;
use crate::discord_data_structs::Pong;
use crate::{
    campaign::Campaign, 
    discord_data_structs::{
        MessageObject, 
        ResponseOject,
        Commands,
        Command
    }
};
use reqwest::{
    Client,
    Url,
    header
};




struct AppState {
    campaigns: Mutex<Vec<Campaign>>
}

enum AppResponse {
    PongInstance(Pong),
    PongInstanceFailed(Pong),
    ResponseInstance(ResponseOject),
    ResponseInstanceFailed(ResponseOject)
}

impl IntoResponse for AppResponse {

    fn into_response(self) -> Response {

        match self {
            AppResponse::PongInstance(p) => {
                (StatusCode::OK, Json(p)).into_response() 
            },
            AppResponse::PongInstanceFailed(p) => {
                (StatusCode::UNAUTHORIZED, Json(p)).into_response()
            },
            AppResponse::ResponseInstance(r) => {
                (StatusCode::OK, Json(r)).into_response()
            },
            AppResponse::ResponseInstanceFailed(r) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(r)).into_response()
            }
            
        }

    }
}

async fn install_commands(){
    let discord_app_id = env::var("DISCORD_APP_ID")
        .expect("DISCORD_APP_ID env variable must be set");
    let discord_token: String = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN env variable must be set");

    let commands_path = Path::new("commands.json");
    
    let mut commands_string: String = fs::read_to_string(&commands_path).expect("could not read commands.json file");

    let mut current_commands: Commands = serde_json::from_str(&commands_string).expect("commands.json file not correctly formatted");

    let client: Client = Client::new();

    let endpoint = format!("https://discord.com/api/v10/applications/{}/commands", discord_app_id);
    let endpoint = Url::from_str(&endpoint)
        .expect("could not create url for installing dicord app commands");

    let auth: header::HeaderName =  header::HeaderName::from_str("Authorization")
        .expect("could not make Autheration header");

    let auth_val: String = format!("Bot {}", discord_token);
    let auth_val: header::HeaderValue = header::HeaderValue::from_str(&auth_val)
        .expect("coudl not create auth header value");

    let mut headermap: header::HeaderMap = HeaderMap::new();
    headermap.insert(auth, auth_val);

    for command in current_commands.commands{
        log::info!("installing command {}", command.name);
        let res = client.post(endpoint.clone())
            .headers(headermap.clone())
            .json(&command)
            .send()
            .await;       

        match res {
            Ok(response) => {
                log::info!("successfully installed command {}\n{:?}",command.name, response )
            },
            Err(e) => {
                log::error!("could not install command {}\n{:?}", command.name, e);
            }
        } 
    }

}
#[tokio::main]
async fn main() {
    env_logger::init();
    
    log::info!("starting discord dm bot");

    let mut campaigns: Vec<Campaign> = Vec::new();
    
    let mut app_state = Arc::new(
        AppState {
            campaigns: Mutex::new(campaigns)
        }
    );

    //install commands
    install_commands().await;
 
    let app = Router::new()
        .route("/interactions", post(pong))
        .route("/init", post(init))
        .route("/action", post(action))
        .with_state(app_state);

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pong(header: HeaderMap, body: Body) -> impl IntoResponse {
    
    log::info!("VERIFICAITON OF PING BEGIN");
    log::debug!("{:?}", body);

    let pong = Pong { r#type: 1 };
    let body_bytes: Bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(e) => {
            log::error!("could not convert body to bytes\n{}", e);
            return AppResponse::PongInstance(pong) 
        }
    };

    let ping_verifier: PingVerifier = PingVerifier::new();

    let bytes_to_verify: Bytes = body_bytes.clone();
    let payload_sig = match ping_verifier.prepare(&header, bytes_to_verify)  {
        Ok(p_s) => p_s,
        Err(e) => {
            log::error!("unable to create payload and signature for verification\n{}", e);
            return AppResponse::PongInstanceFailed(pong) 
        }
    };

    if !ping_verifier.verify(&payload_sig.0, &payload_sig.1) {
        return AppResponse::PongInstanceFailed(pong) 
    }

    log::info!("VERIFICAITON OF PING SUCCESSFUL");
    log::debug!("body bytes: {:?}", body_bytes);
    let body_json: Json<Interaction>   = match Json::from_bytes(&body_bytes){
        Ok(interaction) => interaction,
        Err(e) => {
            let message: String = String::from("500 unable to process request body");
            log::error!("{}\n {}",message,  e);
            let response = ResponseOject::new(message);
            return AppResponse::ResponseInstanceFailed(response) 
        }
    };
    
    match body_json.r#type {
        1 => AppResponse::PongInstance(pong),
        2 => {
            let f = std::fs::File::create("appcomand.json").unwrap();
            let mut buf = std::io::BufWriter::new(f);
            serde_json::to_writer_pretty(&mut buf, &body_json.0).unwrap();
            let message = String::from("app command acepted");
            let r = ResponseOject::new(message);
            AppResponse::ResponseInstance(r)
        },
        3 => {
            let f = std::fs::File::create("message_command.json").unwrap();
            let mut buf = std::io::BufWriter::new(f);
            serde_json::to_writer_pretty(&mut buf, &body_json.0 ).unwrap();
            let message = String::from("app command acepted");
            let r = ResponseOject::new(message);
            AppResponse::ResponseInstance(r)
        },
        _ => {
            let message = String::from("unable to process request");
            let r = ResponseOject::new(message);
            AppResponse::ResponseInstanceFailed(r)
        } 
    }

}


async fn init(
    app_state: State<Arc<AppState>>, 
    header: HeaderMap, 
    body: Json<Interaction> ) -> impl IntoResponse {

    let interaction_type = body.r#type;
    let channel_id = match &body.channel_id{
        Some(c) => c.clone(),
        None => String::from("")
    };
    

    {
        log::debug!("not ping interation");
        match app_state.campaigns.lock() {
            Ok(lock) => {
                for campaign in &*lock{
                    log::debug!("{:?}", campaign);
                    if campaign.channel_id == channel_id {
                        if campaign.active == true { 
                            let res_message = MessageObject {
                                content: String::from("A campaign for this channel already exisits and is currently active")
                            };

                            let res_object = ResponseOject{
                                r#type: 4,
                                data: Some(res_message)
                            };
                            return (StatusCode::OK, Json(res_object))
 
                        }else {
                            let res_message: MessageObject = MessageObject { 
                                content: String::from("A campaign for this channel already exists, but is not active. use command /start to begin!")  
                            };
                            let res_object: ResponseOject = ResponseOject { 
                                r#type: 4, 
                                data: Some(res_message) 
                            };

                            return (StatusCode::OK, Json(res_object))
                        }
                    }
                }
                
            },
            Err(e) => {
                log::error!("unable to obtain lock for app state");
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseOject{ r#type: 255, data: None}))
            }
        };
         
    }
    {
        log::info!("Creating new campaign for channel {}", channel_id);

        match app_state.campaigns.lock().as_mut() {
            Ok(lock) => {
                let campaign: Campaign = Campaign::new(&channel_id);

                lock.push(campaign);

                let mes_obj: MessageObject = MessageObject { 
                    content: String::from("new campaign created for channel! use command /join to joing the campaign. use command /start to beign campaign")
                };

                let res_obj: ResponseOject = ResponseOject { 
                    r#type: 4, 
                    data: Some(mes_obj) 
                };

                return (StatusCode::OK, Json(res_obj))
            },
             Err(e) => {
                log::error!("unable to obtain lock for app state");
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseOject{ r#type: 255, data: None}))
            }
        }
    }
}

async fn start() {

}

async fn join() {
    
}

async fn action(
    app_state: State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<Interaction>
    ) -> impl IntoResponse {

    let mut file_name: String = String::from("application_test.json");
    let interaction_type = body.r#type;



    let p = Path::new(&file_name);

    log::info!("opening file {}", p.display());
    
    let mut file = match File::create(p){
        Ok(f) => f,
        Err(e) => panic!("could not create file {}\n{}", p.display(), e)
        
    };
    
    let buf = serde_json::to_string_pretty(&body).expect("couldn't convert struct to string");
    match file.write_all(&buf.as_bytes()){
        Ok(a) => {
            log::info!("file written");
            (StatusCode::OK, Json(discord_data_structs::ResponseOject { r#type: 1, data: None}))
        },
        Err(e) => {
            log::error!("couldn't write to file {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(discord_data_structs::ResponseOject { r#type: 1, data: None}))
        }
    }


}
