use crate::Config;
use itertools::Itertools;
use route_recognizer::Router;

#[derive(Clone, Debug)]
pub enum AssetType<'a> {
    PortraitSheet,
    PortraitRecolorSheet,
    Portrait(&'a str),
    PortraitFlipped(&'a str),
    SpriteAnimDataXml,
    SpriteZip,
    SpriteRecolorSheet,
    SpriteAnim(&'a str),
    SpriteOffsets(&'a str),
    SpriteShadows(&'a str),
}

pub fn get_url(
    asset_type: AssetType,
    this_srv_url: &str,
    monster_id: i32,
    path_to_form: &[i32],
) -> String {
    let assets_srv_url = Config::GitAssetsUrl.get();
    let mut form_joined = path_to_form.iter().map(|v| format!("{:04}", v)).join("/");
    if !form_joined.is_empty() {
        form_joined = format!("/{}", form_joined);
    }

    match asset_type {
        AssetType::PortraitSheet => {
            format!(
                "{}/assets/{:04}{}/portrait_sheet.png",
                this_srv_url, monster_id, form_joined
            )
        }
        AssetType::PortraitRecolorSheet => {
            format!(
                "{}/assets/{:04}{}/portrait_recolor_sheet.png",
                this_srv_url, monster_id, form_joined
            )
        }
        AssetType::Portrait(emotion) => {
            format!(
                "{}/portrait/{:04}{}/{}.png",
                assets_srv_url,
                monster_id,
                form_joined,
                up(emotion)
            )
        }
        AssetType::PortraitFlipped(emotion) => {
            format!(
                "{}/portrait/{:04}{}/{}^.png",
                assets_srv_url,
                monster_id,
                form_joined,
                up(emotion)
            )
        }
        AssetType::SpriteAnimDataXml => {
            format!(
                "{}/sprite/{:04}{}/AnimData.xml",
                assets_srv_url, monster_id, form_joined
            )
        }
        AssetType::SpriteZip => {
            format!(
                "{}/assets/{:04}{}/sprites.zip",
                this_srv_url, monster_id, form_joined
            )
        }
        AssetType::SpriteRecolorSheet => {
            format!(
                "{}/assets/{:04}{}/sprite_recolor_sheet.png",
                this_srv_url, monster_id, form_joined
            )
        }
        AssetType::SpriteAnim(action) => {
            format!(
                "{}/sprite/{:04}{}/{}-Anim.png",
                assets_srv_url,
                monster_id,
                form_joined,
                up(action)
            )
        }
        AssetType::SpriteOffsets(action) => {
            format!(
                "{}/sprite/{:04}{}/{}-Offsets.png",
                assets_srv_url,
                monster_id,
                form_joined,
                up(action)
            )
        }
        AssetType::SpriteShadows(action) => {
            format!(
                "{}/sprite/{:04}{}/{}-Shadow.png",
                assets_srv_url,
                monster_id,
                form_joined,
                up(action)
            )
        }
    }
}

/// Matches a URL, if it matches returns a tuple of (monster id, form path, asset type)
pub fn match_url(path: &str) -> Option<(i32, Vec<i32>, AssetType)> {
    let mut router = Router::new();

    router.add(
        "/assets/:monsterid/*formpath/portrait_sheet.png",
        AssetType::PortraitSheet,
    );
    router.add(
        "/assets/:monsterid/*formpath/portrait_recolor_sheet.png",
        AssetType::PortraitRecolorSheet,
    );
    router.add(
        "/assets/:monsterid/*formpath/sprites.zip",
        AssetType::SpriteZip,
    );
    router.add(
        "/assets/:monsterid/*formpath/sprite_recolor_sheet.png",
        AssetType::SpriteRecolorSheet,
    );
    router.add(
        "/assets/:monsterid/portrait_sheet.png",
        AssetType::PortraitSheet,
    );
    router.add(
        "/assets/:monsterid/portrait_recolor_sheet.png",
        AssetType::PortraitRecolorSheet,
    );
    router.add("/assets/:monsterid/sprites.zip", AssetType::SpriteZip);
    router.add(
        "/assets/:monsterid/sprite_recolor_sheet.png",
        AssetType::SpriteRecolorSheet,
    );

    let m = router.recognize(path).ok()?;

    let monster_id = m
        .params()
        .find("monsterid")
        .and_then(|x| x.parse::<i32>().ok())?;
    let form_path = m.params().find("formpath").map(|s| {
        s.split('/')
            .map(|x| x.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
    });
    let form_path = match form_path {
        Some(Ok(x)) => x,
        Some(Err(_)) => return None,
        None => vec![],
    };
    Some((monster_id, form_path, (*m.handler()).clone()))
}

fn up(s: &str) -> String {
    // a bit ugly, but it works for now
    if s == "teary-eyed" {
        return "Teary-Eyed".to_string();
    }
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
