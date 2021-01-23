use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantStatsDTO {
    pub item_0: i32,
    pub item_2: i32,
    pub total_units_healed: i32,
    pub item_1: i32,
    pub largest_multi_kill: i32,
    pub gold_earned: i32,
    pub first_inhibitor_kill: bool,
    pub physical_damage_taken: i64,
    pub node_neutralize_assist: Option<i32>,
    pub total_player_score: i32,
    pub champ_level: i32,
    pub damage_dealt_to_objectives: i64,
    pub total_damage_taken: i64,
    pub neutral_minions_killed: i32,
    pub deaths: i32,
    pub triple_kills: i32,
    pub magic_damage_dealt_to_champions: i64,
    pub wards_killed: i32,
    pub penta_kills: i32,
    pub damage_self_mitigated: i64,
    pub largest_critical_strike: i32,
    pub node_neutralize: Option<i32>,
    pub total_time_crowd_control_dealt: i32,
    pub first_tower_kill: bool,
    pub magic_damage_dealt: i64,
    pub total_score_rank: i32,
    pub node_capture: Option<i32>,
    pub wards_placed: i32,
    pub total_damage_dealt: i64,
    pub time_c_cing_others: i64,
    pub magical_damage_taken: i64,
    pub largest_killing_spree: i32,
    pub total_damage_dealt_to_champions: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub neutral_minions_killed_team_jungle: i32,
    pub total_minions_killed: i32,
    pub first_inhibitor_assist: bool,
    pub vision_wards_bought_in_game: i32,
    pub objective_player_score: i32,
    pub kills: i32,
    pub first_tower_assist: bool,
    pub combat_player_score: i32,
    pub inhibitor_kills: i32,
    pub turret_kills: i32,
    pub participant_id: i32,
    pub true_damage_taken: i64,
    pub first_blood_assist: bool,
    pub node_capture_assist: Option<i32>,
    pub assists: i32,
    pub team_objective: Option<i32>,
    pub altars_neutralized: Option<i32>,
    pub gold_spent: i32,
    pub damage_dealt_to_turrets: i64,
    pub altars_captured: Option<i32>,
    pub win: bool,
    pub total_heal: i64,
    pub unreal_kills: i32,
    pub vision_score: i64,
    pub physical_damage_dealt: i64,
    pub first_blood_kill: bool,
    pub longest_time_spent_living: i32,
    pub killing_sprees: i32,
    pub sight_wards_bought_in_game: i32,
    pub true_damage_dealt_to_champions: i64,
    pub neutral_minions_killed_enemy_jungle: i32,
    pub double_kills: i32,
    pub true_damage_dealt: i64,
    pub quadra_kills: i32,
    pub item_4: i32,
    pub item_3: i32,
    pub item_6: i32,
    pub item_5: i32,
    pub player_score_0: i32,
    pub player_score_1: i32,
    pub player_score_2: i32,
    pub player_score_3: i32,
    pub player_score_4: i32,
    pub player_score_5: i32,
    pub player_score_6: i32,
    pub player_score_7: i32,
    pub player_score_8: i32,
    pub player_score_9: i32,
    pub perk_0: i32,
    pub perk_0_var_1: i32,
    pub perk_0_var_2: i32,
    pub perk_0_var_3: i32,
    pub perk_1: i32,
    pub perk_1_var_1: i32,
    pub perk_1_var_2: i32,
    pub perk_1_var_3: i32,
    pub perk_2: i32,
    pub perk_2_var_1: i32,
    pub perk_2_var_2: i32,
    pub perk_2_var_3: i32,
    pub perk_3: i32,
    pub perk_3_var_1: i32,
    pub perk_3_var_2: i32,
    pub perk_3_var_3: i32,
    pub perk_4: i32,
    pub perk_4_var_1: i32,
    pub perk_4_var_2: i32,
    pub perk_4_var_3: i32,
    pub perk_5: i32,
    pub perk_5_var_1: i32,
    pub perk_5_var_2: i32,
    pub perk_5_var_3: i32,
    pub perk_primary_style: i32,
    pub perk_sub_style: i32,
    pub stat_perk_0: i32,
    pub stat_perk_1: i32,
    pub stat_perk_2: i32,
}