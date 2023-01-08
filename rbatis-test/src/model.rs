
use rbatis::rbatis::Rbatis;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

/// example table
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

/// make a rbatis
pub async fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:pwd%^123456^@114.115.234.236:3306/viligogo").unwrap();
    return rb;
}
