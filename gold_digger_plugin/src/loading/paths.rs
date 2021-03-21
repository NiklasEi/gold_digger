pub struct AssetPaths {
    pub fira_sans: &'static str,
    pub texture_sky: &'static str,
    pub texture_digger: &'static str,
    pub texture_background: &'static str,
    pub texture_border: &'static str,
    pub texture_tank_upgrade: &'static str,
    pub texture_stone: &'static str,
    pub texture_stone_mining: &'static str,
    pub texture_gold: &'static str,
    pub texture_gold_mining: &'static str,
    pub texture_base: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    fira_sans: "fonts/FiraSans-Bold.ttf",
    texture_sky: "textures/none.png",
    texture_base: "textures/base.png",
    texture_digger: "textures/digger.png",
    texture_background: "textures/background.png",
    texture_tank_upgrade: "textures/tank_upgrade.png",
    texture_border: "textures/border.png",
    texture_stone: "textures/stone.png",
    texture_stone_mining: "textures/stone_mining.png",
    texture_gold: "textures/gold.png",
    texture_gold_mining: "textures/gold_mining.png",
};
