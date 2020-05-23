use crate::constants::{
  MAX_S,
  MAX_V
};

fn values_to_rgb(r: f32, g: f32, b: f32) -> (i32, i32, i32) {
  (
    (r * 255.0).round() as i32,
    (g * 255.0).round() as i32,
    (b * 255.0).round() as i32
  )
}

fn value_to_hex_part(value: i32) -> String {
  let part = format!("{:X}", value);
  if part.len() == 1 {
    return format!("0{}", part)
  }

  part
}

// h = [0,360], s = [0,100], v = [0,100]
pub fn hsv_to_rgb(h: i32, s: i32, v: i32) -> (i32, i32, i32) {
  let s_norm = s as f32 / MAX_S;
  let v_norm = v as f32 / MAX_V;

  let t: f32;

	if s == 0 {
		return values_to_rgb(v_norm, v_norm, v_norm);
	}

	let h_sector: f32 = h as f32 / 60.0;			// sector 0 to 5
	let i = h_sector.floor();
	let f = h_sector - i;			// factorial part of h
  let p: f32 = v_norm * (1.0 - s_norm);
	let q: f32 = v_norm * (1.0 - s_norm * f);
  let t: f32 = v_norm * (1.0 - s_norm * (1.0 - f));
  
  if i == 0.0 {
    return values_to_rgb(v_norm, t, p);
  } else if i == 1.0 {
    return values_to_rgb(q, v_norm, p);
  } else if i == 2.0 {
    return values_to_rgb(p, v_norm, t);
  } else if i == 3.0 {
    return values_to_rgb(p, q, v_norm);
  } else if i == 4.0 {
    return values_to_rgb(t, p, v_norm);
  } else {
    return values_to_rgb(v_norm, p, q);
  }
}

pub fn rgb_to_hex(r: i32, g: i32, b: i32) -> String {
  let r_part = value_to_hex_part(r);
  let g_part = value_to_hex_part(g);
  let b_part = value_to_hex_part(b);

  format!("#{}{}{}", r_part, g_part, b_part)
}
