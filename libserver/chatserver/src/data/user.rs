use chrono::NaiveDateTime;

#[repr(u8)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
}

pub struct UserVo {
    pub user_id: String,
    pub user_name: String,
    pub nick_name: String,
    pub password: Option<String>,
    pub gender: Gender,
    pub phone: String,
    pub birthday: Option<i64>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub signature: Option<String>,
    pub face_type: Option<i32>,
    pub custom_face: Option<String>,
    pub custom_face_fmt: Option<String>,
    pub group_info: Option<Vec<u8>>,
    pub register_time: NaiveDateTime,
}
