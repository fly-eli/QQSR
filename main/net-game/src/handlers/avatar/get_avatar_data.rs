use cfg_server::avatar::AvatarConfig;
use net_msg::pb::GetAvatarDataScRsp;
use net_msg::Trait;

pub async fn handle(_req: &[u8]) -> Vec<u8> {
    let cfg = AvatarConfig::from_file("avatar.toml");

    GetAvatarDataScRsp {
        is_get_all: true,
        avatar_list: cfg.get_owned_avatars(),
        retcode: 0,
        unlocked_skin_id_list: Vec::with_capacity(0),
    }
    .encode_to_vec()
}
