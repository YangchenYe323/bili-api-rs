//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/live/info.md

use std::collections::HashMap;

use serde::Deserialize;

use crate::{BiliApiRequest, BiliApiResponse, BiliClient};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LiveRoomInfoResponse {
    Success {
        code: i32,
        data: LiveRoomData,
    },
    Failure {
        code: i32,
        message: String,
        msg: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct LiveRoomData {
    pub uid: i32,
    pub room_id: i32,
    pub short_id: i32,
    pub attention: i32,
    pub online: i32,
    pub is_portrait: bool,
    pub description: String,
    pub live_status: i32,
    pub area_id: i32,
    pub parent_area_id: i32,
    pub parent_area_name: String,
    pub old_area_id: i32,
    pub background: String,
    pub title: String,
    pub user_cover: String,
    pub keyframe: String,
    pub is_strict_room: bool,
    pub live_time: String,
    pub tags: String,
    pub is_anchor: i32,
    pub room_silent_type: String,
    pub room_silent_level: i32,
    pub room_silent_second: i32,
    pub area_name: String,
    pub pardants: Option<String>,
    pub area_pardants: Option<String>,
    pub hot_words: Vec<String>,
    pub hot_words_status: i32,
    pub verify: String,
    pub new_pendants: NewPendants,
    pub up_session: String,
    pub pk_status: i32,
    pub pk_id: i32,
    pub battle_id: i32,
    pub allow_change_area_time: i32,
    pub allow_upload_cover_time: i32,
    pub studio_info: StudioInfo,
}

#[derive(Debug, Deserialize)]
pub struct NewPendants {
    pub frame: Frame,
    pub mobile_frame: Option<Frame>,
    pub badge: Badge,
    pub mobile_badge: Option<Badge>,
}

#[derive(Debug, Deserialize)]
pub struct Frame {
    pub name: String,
    pub value: String,
    pub position: i32,
    pub desc: String,
    pub area: i32,
    pub area_old: i32,
    pub bg_color: String,
    pub bg_pic: String,
    pub use_old_area: bool,
}

#[derive(Debug, Deserialize)]
pub struct Badge {
    pub name: String,
    pub position: i32,
    pub value: String,
    pub desc: String,
}

#[derive(Debug, Deserialize)]
pub struct StudioInfo {
    pub status: i32,
    pub master_list: Vec<String>, // Adjust this based on actual expected type
}

pub fn get_live_room_info<T: BiliClient>(
    client: &T,
    room_id: i32,
) -> std::result::Result<LiveRoomInfoResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";
    let url = format!("{}?room_id={}", API_URL, room_id);
    let request = client.create_request("GET", &url);
    let response = request.execute()?;
    Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RoomInitInfoResponse {
    Success {
        code: i32,
        data: RoomInitData,
    },
    Failure {
        code: i32,
        message: String,
        msg: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct RoomInitData {
    pub room_id: i32,
    pub short_id: i32,
    pub uid: i32,
    pub need_p2p: i32,
    pub is_hidden: bool,
    pub is_locked: bool,
    pub is_portrait: bool,
    pub live_status: i32,
    pub hidden_till: i64,
    pub lock_till: i64,
    pub encrypted: bool,
    pub pwd_verified: bool,
    pub live_time: i64,
    pub room_shield: i32,
    pub is_sp: i32,
    pub special_type: i32,
}

pub fn get_room_init_info<T: BiliClient>(
    client: &T,
    room_id: i32,
) -> std::result::Result<RoomInitInfoResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";
    let url = format!("{}?id={}", API_URL, room_id);
    let request = client.create_request("GET", &url);
    let response = request.execute()?;
    Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StreamerInfoResponse {
    Success {
        code: i32,
        data: StreamerData,
    },
    Failure {
        code: i32,
        message: String,
        msg: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct StreamerData {
    pub info: StreamerInfo,
    pub exp: StreamerExp,
    pub follower_num: i32,
    pub room_id: i32,
    pub medal_name: String,
    pub glory_count: i32,
    pub pendant: String,
    pub link_group_num: i32,
    pub room_news: RoomNews,
}

#[derive(Debug, Deserialize)]
pub struct StreamerInfo {
    pub uid: i32,
    pub uname: String,
    pub face: String,
    pub official_verify: OfficialVerify,
    pub gender: i32,
}

#[derive(Debug, Deserialize)]
pub struct OfficialVerify {
    #[serde(rename = "type")]
    pub type_: i32,
    pub desc: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamerExp {
    pub master_level: MasterLevel,
}

#[derive(Debug, Deserialize)]
pub struct MasterLevel {
    pub level: i32,
    pub color: i32,
    pub current: Vec<i32>,
    pub next: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RoomNews {
    pub content: String,
    pub ctime: String,
    pub ctime_text: String,
}

pub fn get_streamer_info<T: BiliClient>(
    client: &T,
    uid: i32,
) -> std::result::Result<StreamerInfoResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/live_user/v1/Master/info";
    let url = format!("{}?uid={}", API_URL, uid);
    let request = client.create_request("GET", &url);
    let response = request.execute()?;
    Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RoomStatusBatchResponse {
    Success {
        code: i32,
        data: HashMap<String, LiveRoomStatus>,
    },
    Failure {
        code: i32,
        message: String,
        msg: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct LiveRoomStatus {
    pub title: String,
    pub room_id: i32,
    pub uid: i32,
    pub online: i32,
    pub live_time: i64,
    pub live_status: i32,
    pub short_id: i32,
    pub area: i32,
    pub area_name: String,
    pub area_v2_id: i32,
    pub area_v2_name: String,
    pub area_v2_parent_id: i32,
    pub area_v2_parent_name: String,
    pub uname: String,
    pub face: String,
    pub tag_name: String,
    pub tags: String,
    pub cover_from_user: String,
    pub keyframe: String,
    pub lock_till: String,
    pub hidden_till: String,
    pub broadcast_type: i32,
}

pub fn query_room_status_batch<T: BiliClient>(
    client: &T,
    uids: &[i32],
) -> std::result::Result<RoomStatusBatchResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/room/v1/Room/get_status_info_by_uids";
    let uids_query: String = uids
        .iter()
        .map(|&uid| format!("uids[]={}", uid))
        .collect::<Vec<_>>()
        .join("&");
    let url = format!("{}?{}", API_URL, uids_query);
    let request = client.create_request("GET", &url);
    let response = request.execute()?;
    Ok(response.deserialize_json().unwrap())
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LiveRoomPlayInfoResponse {
    Success {
        code: i32,
        data: LiveRoomPlayData,
        ttl: i32,
    },
    Failure {
        code: i32,
        message: String,
        ttl: i32,
    },
}

#[derive(Debug, Deserialize)]
pub struct LiveRoomPlayData {
    pub room_id: i32,
    pub short_id: i32,
    pub uid: i32,
    pub is_hidden: bool,
    pub is_locked: bool,
    pub is_portrait: bool,
    pub live_status: i32,
    pub hidden_till: i64,
    pub lock_till: i64,
    pub encrypted: bool,
    pub pwd_verified: bool,
    pub live_time: i64,
    pub room_shield: i32,
    pub all_special_types: Vec<i32>, // Assuming String for simplicity
    pub playurl_info: Option<PlayurlInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PlayurlInfo {
    pub conf_json: String,
    pub playurl: Option<Playurl>,
}

#[derive(Debug, Deserialize)]
pub struct Playurl {
    pub cid: i32,
    pub g_qn_desc: Vec<QualityDescription>,
    pub stream: Vec<StreamInfo>,
    pub p2p_data: P2PData,
}

#[derive(Debug, Deserialize)]
pub struct QualityDescription {
    pub qn: i32,
    pub desc: String,
    pub hdr_desc: String,
    pub attr_desc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StreamInfo {
    pub protocol_name: String,
    pub format: Vec<FormatInfo>,
}

#[derive(Debug, Deserialize)]
pub struct FormatInfo {
    pub format_name: String,
    pub master_url: String,
    pub codec: Vec<CodecInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CodecInfo {
    pub codec_name: String,
    pub current_qn: i32,
    pub accept_qn: Vec<i32>,
    pub base_url: String,
    pub url_info: Vec<UrlInfo>,
    pub hdr_qn: Option<i32>,
    pub dolby_type: i32,
    pub attr_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UrlInfo {
    pub host: String,
    pub extra: String,
    pub stream_ttl: i32,
}

#[derive(Debug, Deserialize)]
pub struct P2PData {
    pub p2p: bool,
    pub p2p_type: i32,
    pub m_p2p: bool,
    pub m_servers: Option<Vec<String>>, // Assuming Vec<String> for simplicity
}

pub fn get_live_room_play_info<T: BiliClient>(
    client: &T,
    room_id: i32,
    protocols: &[&str],
    formats: &[&str],
    codecs: &[&str],
    qn: i32,
) -> std::result::Result<LiveRoomPlayInfoResponse, <T::Request as BiliApiRequest>::Error> {
    const API_URL: &str = "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo";
    let protocols_str = protocols.join(",");
    let formats_str = formats.join(",");
    let codecs_str = codecs.join(",");
    let url = format!(
        "{}?room_id={}&protocol={}&format={}&codec={}&qn={}",
        API_URL, room_id, protocols_str, formats_str, codecs_str, qn
    );
    let request = client.create_request("GET", &url);
    let response = request.execute()?;
    // println!("{}", response.deserialize_json::<serde_json::Value>().unwrap());
    // unimplemented!()
    Ok(response.deserialize_json().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_live_room_info() {
        let agent = ureq::agent();
        // Success: a valid live room
        assert!(matches!(
            get_live_room_info(&agent, 1029).unwrap(),
            LiveRoomInfoResponse::Success { .. }
        ));
        // Failure: non-existent live room
        assert!(matches!(
            get_live_room_info(&agent, 1231232412).unwrap(),
            LiveRoomInfoResponse::Failure { .. }
        ));
    }

    #[test]
    fn test_get_room_init_info() {
        let agent = ureq::agent();
        // Success: a valid live room
        assert!(matches!(
            get_room_init_info(&agent, 1029).unwrap(),
            RoomInitInfoResponse::Success { .. }
        ));
        // Failure: non-existent live room
        assert!(matches!(
            get_room_init_info(&agent, 1231232412).unwrap(),
            RoomInitInfoResponse::Failure { .. }
        ));
    }

    #[test]
    fn test_get_streamer_info() {
        let agent = ureq::agent();
        // Success: a valid uid
        assert!(matches!(
            get_streamer_info(&agent, 697737710).unwrap(),
            StreamerInfoResponse::Success { .. }
        ));
        // Failure: non-existent uid
        assert!(matches!(
            get_streamer_info(&agent, 0).unwrap(),
            StreamerInfoResponse::Failure { .. }
        ));
    }

    #[test]
    fn test_query_room_status_batch() {
        let agent = ureq::agent();
        let uids = [697737710, 540564177];
        let invalid_uids = [44444444];
        // Success: a valid uid
        assert!(matches!(
            query_room_status_batch(&agent, &uids).unwrap(),
            RoomStatusBatchResponse::Success { .. }
        ));
        // Failure: non-existent uid
        assert!(matches!(
            query_room_status_batch(&agent, &invalid_uids).unwrap(),
            RoomStatusBatchResponse::Failure { .. }
        ));
    }

    #[test]
    fn test_get_live_room_play_info() {
        let agent = ureq::agent();
        println!(
            "{:?}",
            get_live_room_play_info(
                &agent,
                23058,
                &["0", "1"],
                &["0", "1", "2"],
                &["0", "1"],
                1000
            )
            .unwrap()
        );
        // 直播间不存在
        println!(
            "{:?}",
            get_live_room_play_info(
                &agent,
                44444444,
                &["0", "1"],
                &["0", "1", "2"],
                &["0", "1"],
                1000
            )
            .unwrap()
        );
        println!(
            "{:?}",
            get_live_room_play_info(
                &agent,
                23058,
                &["arbitrary_parameter"],
                &["abcd"],
                &["another_arong"],
                1000
            )
            .unwrap()
        );
    }
}
