pub const COMBATLOG_OBJECT_AFFILIATION_MINE: i64 = 0x00000001;
pub const COMBATLOG_OBJECT_REACTION_FRIENDLY: i64 = 0x00000010;
pub const COMBATLOG_OBJECT_CONTROL_PLAYER: i64 = 0x00000100;
pub const COMBATLOG_OBJECT_TYPE_PLAYER: i64 = 0x00000400;

pub const COMBATLOG_FILTER_ME: i64 = COMBATLOG_OBJECT_AFFILIATION_MINE | COMBATLOG_OBJECT_REACTION_FRIENDLY | COMBATLOG_OBJECT_CONTROL_PLAYER | COMBATLOG_OBJECT_TYPE_PLAYER;

pub const NIL_WOW_GUID: &'static str = "0000000000000000";