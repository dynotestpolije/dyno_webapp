use dyno_core::{
    dynotests::DynoTest,
    log,
    users::{UserResponse, UserUpdate},
    ActiveResponse, ApiResponse, BufferData, DynoConfig, DynoErr, DynoResult, HistoryResponse,
};
use gloo::{file::Blob, net::http::Request, utils::document};
use web_sys::MouseEvent;
use yew::UseStateSetter;

use crate::state::AppState;

pub async fn fetch_dashboard(state: &mut AppState, token: impl AsRef<str>) {
    let fetched = match Request::get("/api/auth/me")
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.ok() => resp
            .json::<ApiResponse<UserResponse>>()
            .await
            .map(|x| x.payload)
            .ok(),
        Err(err) => {
            log::error!("{err}");
            None
        }
        _ => None,
    };
    state.set_me(fetched);
}
pub async fn fetch_dyno(state: &mut AppState, token: impl AsRef<str>) {
    match Request::get(if state.user_session().is_some_and(|x| x.role.is_admin()) {
        "/api/dyno?all=true&admin=true"
    } else {
        "/api/dyno?all=true"
    })
    .header("Authorization", token.as_ref())
    .send()
    .await
    {
        Ok(resp) if resp.ok() => match resp
            .json::<ApiResponse<Vec<DynoTest>>>()
            .await
            .map(|x| x.payload)
        {
            Ok(data) => state.get_data_mut().set_dyno(data),
            Err(err) => log::error!("{err}"),
        },
        Err(err) => log::error!("{err}"),
        _ => {}
    }
}

pub async fn fetch_delete_user(token: impl AsRef<str>, user_id: i64) -> bool {
    let url = format!("/api/users/{user_id}");
    match Request::delete(&url)
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.ok() => true,
        Err(err) => {
            log::error!("{err}");
            false
        }
        _ => false,
    }
}

pub async fn fetch_user(state: &mut AppState, token: impl AsRef<str>) {
    let url = if state.user_session().is_some_and(|x| x.role.is_admin()) {
        "/api/users"
    } else {
        return;
    };

    match Request::get(url)
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.ok() => match resp
            .json::<ApiResponse<Vec<UserResponse>>>()
            .await
            .map(|x| x.payload)
        {
            Ok(data) => state.get_data_mut().set_users(data),
            Err(err) => log::error!("{err}"),
        },
        Err(err) => log::error!("{err}"),
        _ => {}
    }
}

pub async fn fetch_infos(state: &mut AppState, token: impl AsRef<str>) {
    let url = if state.user_session().is_some_and(|x| x.role.is_admin()) {
        "/api/info?all=true&admin=true"
    } else {
        return;
    };

    match Request::get(url)
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.ok() => match resp
            .json::<ApiResponse<Vec<DynoConfig>>>()
            .await
            .map(|x| x.payload)
        {
            Ok(data) => state.get_data_mut().set_infos(data),
            Err(err) => log::error!("{err}"),
        },
        Err(err) => log::error!("{err}"),
        _ => {}
    }
}

pub async fn fetch_info_byid(token: impl AsRef<str>, id: i64) -> Option<DynoConfig> {
    let url = format!("/api/info?id={}", id);
    match Request::get(&url)
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.ok() => resp
            .json::<ApiResponse<DynoConfig>>()
            .await
            .map(|x| x.payload)
            .ok(),
        Err(err) => {
            log::error!("{err}");
            None
        }
        _ => None,
    }
}

pub async fn fetch_histories(state: &mut AppState, token: impl AsRef<str>) {
    let url = if state.user_session().is_some_and(|x| x.role.is_admin()) {
        "/api/history?all=true&admin=true"
    } else {
        return;
    };

    match Request::get(url)
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.ok() => match resp
            .json::<ApiResponse<Vec<HistoryResponse>>>()
            .await
            .map(|x| x.payload)
        {
            Ok(data) => state.get_data_mut().set_last_usage(data),
            Err(err) => log::error!("{err}"),
        },
        Err(err) => log::error!("{err}"),
        _ => {}
    }
}

pub async fn fetch_status(active: &UseStateSetter<Option<ActiveResponse>>, token: impl AsRef<str>) {
    let fetched_active = match Request::get("/api/active")
        .header("Authorization", token.as_ref())
        .send()
        .await
    {
        Ok(resp) if resp.ok() => resp
            .json::<ApiResponse<ActiveResponse>>()
            .await
            .map(|x| x.payload)
            .ok(),
        Err(err) => {
            log::error!("{err}");
            None
        }
        _ => None,
    };
    active.set(fetched_active)
}

pub async fn fetch_data_dyno(file_url: impl AsRef<str>, token: impl AsRef<str>) -> BufferData {
    match Request::get(file_url.as_ref())
        .query([("tp", "json")])
        .header("Authorization", token.as_ref())
        .send()
        .await
        .map_err(DynoErr::api_error)
    {
        Ok(response) => match response
            .json::<ApiResponse<BufferData>>()
            .await
            .map_err(DynoErr::api_error)
        {
            Ok(json) => json.payload,
            Err(err) => {
                log::error!("{err}");
                Default::default()
            }
        },
        Err(err) => {
            log::error!("{err}");
            Default::default()
        }
    }
}

pub async fn fetch_and_save(
    file_url: impl AsRef<str>,
    filetype: impl AsRef<str>,
    token: impl AsRef<str>,
) -> DynoResult<()> {
    let response = Request::get(file_url.as_ref())
        .query([("tp", filetype.as_ref())])
        .header("Authorization", token.as_ref())
        .send()
        .await
        .map_err(DynoErr::api_error)?;

    let content_disposition = response
        .headers()
        .get("Content-Disposition")
        .ok_or(DynoErr::api_error("no Content-Disposition header"))?;
    let filename = content_disposition
        .split('=')
        .last()
        .map(|name| name.trim_matches(|c| c == '"' || c == ' '))
        .ok_or(DynoErr::api_error("Failed get last string split"))?;

    response
        .binary()
        .await
        .map_err(DynoErr::api_error)
        .map(move |blob| {
            let blob = Blob::new(blob.as_slice());
            let blob = blob.into();
            let url = web_sys::Url::create_object_url_with_blob(&blob)
                .expect("Failed create object url on Download");

            let anchor = document()
                .create_element("a")
                .expect("Failed create  tag <a> element");
            anchor
                .set_attribute("href", &url)
                .expect("Failed set attribute 'href' in tag <a>");
            anchor
                .set_attribute("download", filename)
                .expect("Failed set attribute 'download' in tag <a>");
            anchor
                .set_attribute("style", "display: none")
                .expect("Failed set attribute 'display' in tag <a>");
            let body = document()
                .body()
                .expect("Failed to get body element in document");
            body.append_child(&anchor)
                .expect("Failed to append anchor on body element");
            let event = MouseEvent::new("click").expect("Failed to  create MouseEvent");
            anchor
                .dispatch_event(&event)
                .expect("Failed dispatch event on anchor Element");
            body.remove_child(&anchor)
                .expect("Failed to remove anchor element in body");

            web_sys::Url::revoke_object_url(&url).expect("Failed to revoce object url");
        })
}

pub async fn update_user(id: i64, user: UserUpdate, token: impl AsRef<str>) -> DynoResult<i64> {
    let url = format!("/users/{id}");
    let resp = Request::get(&url)
        .header("Authorization", token.as_ref())
        .json(&user)
        .map_err(DynoErr::api_error)?
        .send()
        .await
        .map_err(DynoErr::api_error)?;

    if resp.ok() {
        resp.json::<i64>().await.map_err(DynoErr::api_error)
    } else {
        let err = resp.text().await.map_err(DynoErr::api_error)?;
        Err(DynoErr::api_error(err))
    }
}
