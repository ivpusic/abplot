extern crate serialize;

use serialize::{json};

#[deriving(Decodable, Show, Clone)]
pub struct LinesGraph {
    pub title: String,
    pub file: String,
}

#[deriving(Decodable, Show, Clone)]
pub struct PointsGraph {
    pub title: String,
    pub file: String,
}


#[deriving(Decodable, Show, Clone)]
pub struct BoxesGraph {
    pub title: String,
    pub file: String,
}

#[deriving(Decodable, Show, Clone)]
pub struct GraphDefinitions {
    pub lines: Option<LinesGraph>,
    pub points: Option<PointsGraph>,
    pub boxes: Option<BoxesGraph>
}

#[deriving(Decodable, Show, Clone)]
pub struct ServerDef {
    pub address: String,
    pub title: String,
    pub datafile: Option<String>
}

#[deriving(Decodable, Show, Clone)]
pub struct JsonConfig<'s> {
    pub urls: Vec<ServerDef>,
    pub n: int,
    pub c: int,
    pub graphs: GraphDefinitions
}

pub fn parse(config: String) -> JsonConfig<'static> {
    let cfg: JsonConfig = json::decode(config.as_slice()).unwrap();
    return cfg
}
