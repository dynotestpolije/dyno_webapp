use crate::schema::dyno_info;
use dyno_core::chrono::NaiveDateTime;
use dyno_core::{serde, Cylinder, DynoConfig, MotorType, Numeric, Stroke};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Selectable,
)]
#[serde(crate = "serde")]
#[diesel(table_name = dyno_info)]
pub struct DynoInfo {
    pub id: i64,
    pub motor_type: i16,
    pub name: Option<String>,
    pub cc: Option<i16>,
    pub cylinder: Option<i16>,
    pub stroke: Option<i16>,
    pub diameter_roller: Option<f32>,
    pub diameter_roller_beban: Option<f32>,
    pub diameter_gear_encoder: Option<f32>,
    pub diameter_gear_beban: Option<f32>,
    pub jarak_gear: Option<f32>,
    pub berat_beban: Option<f32>,
    pub gaya_beban: Option<f32>,
    pub keliling_roller: Option<f32>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl DynoInfo {
    #[inline]
    pub fn into_response(self) -> DynoConfig {
        DynoConfig {
            motor_type: MotorType::from(self.motor_type as u8),
            diameter_roller: self.diameter_roller.unwrap_or_default().into(),
            diameter_roller_beban: self.diameter_roller_beban.unwrap_or_default().into(),
            diameter_gear_encoder: self.diameter_gear_encoder.unwrap_or_default().into(),
            diameter_gear_beban: self.diameter_gear_beban.unwrap_or_default().into(),
            jarak_gear: self.jarak_gear.unwrap_or_default().into(),
            berat_beban: self.berat_beban.unwrap_or_default().into(),
            gaya_beban: self.gaya_beban.unwrap_or_default().into(),
            keliling_roller: self.keliling_roller.unwrap_or_default().into(),
            motor_info: dyno_core::MotorInfo {
                name: self.name.unwrap_or_default(),
                cc: self.cc.unwrap_or_default() as _,
                cylinder: Cylinder::from(self.cc.unwrap_or_default() as u8),
                stroke: Stroke::from(self.stroke.unwrap_or_default() as u8),
                transmition: Default::default(),
            },
            ..Default::default()
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Default, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[serde(crate = "serde")]
#[diesel(table_name = dyno_info)]
pub struct NewDynoInfo {
    pub motor_type: i16,
    pub name: Option<String>,
    pub cc: Option<i16>,
    pub cylinder: Option<i16>,
    pub stroke: Option<i16>,
    pub diameter_roller: Option<f32>,
    pub diameter_roller_beban: Option<f32>,
    pub diameter_gear_encoder: Option<f32>,
    pub diameter_gear_beban: Option<f32>,
    pub jarak_gear: Option<f32>,
    pub berat_beban: Option<f32>,
    pub gaya_beban: Option<f32>,
    pub keliling_roller: Option<f32>,
}

impl NewDynoInfo {
    pub fn from_dyno_config(
        DynoConfig {
            motor_type,
            diameter_roller,
            diameter_roller_beban,
            diameter_gear_encoder,
            diameter_gear_beban,
            jarak_gear,
            berat_beban,
            gaya_beban,
            keliling_roller,
            motor_info,
            ..
        }: DynoConfig,
    ) -> Self {
        match motor_type {
            MotorType::Electric => Self {
                motor_type: MotorType::Electric as i16,
                name: Some(motor_info.name),
                diameter_roller: Some(diameter_roller.to_f32()),
                diameter_roller_beban: Some(diameter_roller_beban.to_f32()),
                diameter_gear_encoder: Some(diameter_gear_encoder.to_f32()),
                diameter_gear_beban: Some(diameter_gear_beban.to_f32()),
                jarak_gear: Some(jarak_gear.to_f32()),
                berat_beban: Some(berat_beban.to_f32()),
                gaya_beban: Some(gaya_beban.to_f32()),
                keliling_roller: Some(keliling_roller.to_f32()),
                ..Default::default()
            },
            MotorType::Engine => Self {
                motor_type: MotorType::Engine as _,
                name: Some(motor_info.name),
                diameter_roller: Some(diameter_roller.to_f32()),
                diameter_roller_beban: Some(diameter_roller_beban.to_f32()),
                diameter_gear_encoder: Some(diameter_gear_encoder.to_f32()),
                diameter_gear_beban: Some(diameter_gear_beban.to_f32()),
                jarak_gear: Some(jarak_gear.to_f32()),
                berat_beban: Some(berat_beban.to_f32()),
                gaya_beban: Some(gaya_beban.to_f32()),
                keliling_roller: Some(keliling_roller.to_f32()),
                cc: Some(motor_info.cc as _),
                cylinder: Some(motor_info.cylinder as _),
                stroke: Some(motor_info.stroke as _),
            },
        }
    }
}

impl From<DynoConfig> for NewDynoInfo {
    fn from(value: DynoConfig) -> Self {
        Self::from_dyno_config(value)
    }
}
