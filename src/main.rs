use tokio::{self, net::TcpListener};
use log;
use std::fmt::format;
use std::fs;
use std::str::FromStr;
use std::{
    fs::File, 
    io::Write,
    path::Path,
    env
};
use bytes::{Bytes, buf};
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
use crate::discord_data_structs::{AppCommand, DataValues, Pong};
use crate::player::Player;
use crate::{
    campaign::Campaign, 
    discord_data_structs::{
        MessageObject, 
        ResponseOject,
        Commands,
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
        .with_state(app_state);

    
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pong(app_state: State<Arc<AppState>>, header: HeaderMap, body: Body) -> impl IntoResponse {

     
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

    let data_to_process = body_json.0.clone();

    match body_json.r#type {
        1 => AppResponse::PongInstance(pong),
        2 => {
            match body_json.0.data {
                Some(d) => {

                    let data = match d {
                        DataValues::App(data) => data,
                        _ => {
                            log::info!("not supported");
                            AppCommand {
                                name: String::from("NA"),
                                r#type: 255,
                                id: String::from("NA")
                            }     
                        }
                    };
                    log::debug!("processing command {}", data.name);
                    let response: ResponseOject = match data.name.as_str() {
                        
                        "init" => init(app_state, &data_to_process).await,
                        "join" => join(app_state, &data_to_process).await,
                        "action" => gen_action_modal().await,       
                        _ => {
                            
                            let message: String = format!("{} command not implmented", data.name);
                            log::debug!("{}", message);
                            ResponseOject::new(message)
                        }
                    };
                    AppResponse::ResponseInstance(response)
                },
                None => {
                    let message: String = String::from("no data found");
                    let r: ResponseOject = ResponseOject::new(message);
                    AppResponse::ResponseInstanceFailed(r)
                }
            }
        },
        5 => AppResponse::ResponseInstance(action(app_state, &data_to_process).await),
        _ => {
            let message = String::from("unable to process request");
            let r = ResponseOject::new(message);
            AppResponse::ResponseInstanceFailed(r)
        } 
    }

}


async fn init(
    app_state: State<Arc<AppState>>, 
    body: &Interaction) -> ResponseOject {

    let channel_id = match &body.channel_id {
        Some(c) => c,
        None => "" 
    };

    log::info!("checking if channel {} is active", channel_id); 
    let message: String;
    {
        match app_state.campaigns.lock().as_mut() {
            Ok(lock) => {
                for campaign in lock.iter(){
                    log::debug!("{:?}", campaign);
                    if campaign.channel_id == channel_id {
                        if campaign.active == true { 
                            message = String::from("A campaign for this channel already exisits and is currently active");
                            log::error!("{}", message);
                            let res_object = ResponseOject::new(message);
                            return res_object
 
                        }else {
                            message = String::from("A campaign for this channel already exists, but is not active. use command /start to begin!");
                            log::debug!("{}",message);  
                            let res_object: ResponseOject = ResponseOject::new(message); 
                            return res_object
                        }
                    }
                }

                log::info!("channel not found in campaigns - creating new campaign");

                let new_campaign: Campaign = Campaign::new(channel_id);
                log::debug!("campaigns before {:?}", lock);
                lock.push(new_campaign);
                log::debug!("campaings after {:?}", lock);

                message = String::from("created new campaign for channel\nused command /join to join the campaign\nuse command /start to begin."); 
                let res_object:ResponseOject = ResponseOject::new(message);
                return res_object;
            },
            Err(e) => {
                log::error!("unable to obtain lock for app state {}", e);
                return ResponseOject{ r#type: 255, data: None}
            }
        };
    }
}



async fn join(
    app_state: State<Arc<AppState>>,
    body: &Interaction) -> ResponseOject {
    
    let message: String;
    let user = match &body.user {
        Some(u) => u,
        None => {
            log::debug!("User data not found checking member data");
            match &body.member {
                Some(m) => {
                    match &m.user {
                        Some(m_user) => m_user,
                        None => {
                            log::error!("no user data in interaction payload");
                            message = String::from("unable to add user to campaign");
                            return ResponseOject::new(message)
                        }
                    }
                },
                None => {
                    log::error!("no user data in interaction payload");
                    message = String::from("unable to add user to campaign");
                    return ResponseOject::new(message)
                }
            }
        }
    };

    let channel_id = match &body.channel_id {
        Some(channel) => channel,
        None => {
            log::error!("no channel id found in request body");
            message = String::from("could no process command");
            return ResponseOject::new(message)
        }
    };

    match app_state.campaigns.lock().as_mut() {
        Ok(lock) => {
            log::debug!("got lock for join command");
            for campaign in lock.iter_mut() {
                if campaign.channel_id.as_str() == channel_id {
                    log::debug!("found campaign {}", channel_id);
                    for player in campaign.players.iter() {
                        if player.id == user.id {
                            let message = format!("Player {} has already joinned the campaign.\nuse command /start to beign!", player.display_name);
                            log::info!("{}", message);
                            return ResponseOject::new(message)                
                        }
                    }

                    log::info!("user {} not found in campaign. creating new user",user.global_name);

                    let new_player = Player::new(user.id.clone(), user.global_name.clone());

                    campaign.players.push(new_player);

                    log::info!("player {} added to campaign {}", user.global_name, campaign.channel_id);
                    let mut message = format!("Welcome @{}! you have successfuly joined the campaign!\nCurrent Players:\n", user.global_name);

                    let current_players = campaign.players.iter().map(|p| format!("{}\n", p.display_name)).collect::<String>(); 
                    message += &current_players;

                    return ResponseOject::new(message)

                }
            }
            log::error!("campaign {} not found in app state", channel_id);
            message = String::from("no campaign found for this channel\nuse /init command to start a new campaign");
            return ResponseOject::new(message)
        },
        Err(e) => {
            log::error!("unable to obtain mutex log for join command\n{}", e);
            message = String::from("sorry, could process your request. try again later");
            return ResponseOject::new(message)
        }
    };
}

async fn gen_action_modal() -> ResponseOject {
    log::info!("action command received - generating action modal response");

    let r = ResponseOject::new_action_modal();
    log::debug!("response object {:?}", serde_json::to_string(&r).unwrap());
    r
}

async fn action(
    app_state: State<Arc<AppState>>,
    body: &Interaction
    ) -> ResponseOject {


    ResponseOject::new(String::from("action command received"))

}
