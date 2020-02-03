fn main() {
  let (num_people, prob) = how_many(0.5);
  println!(
    "If {} people are in a room, there is a {} probability that two of them share a birthday.",
    num_people, prob
  );
}

/// Computes the number of people necessary such that there is a probability greater than `prob`
/// that at least two of them share a birthday.
fn how_many(prob: f64) -> (u32, f64) {
  let mut inv_prob = 1f64;

  for i in 0.. {
    inv_prob *= (365 - i) as f64 / 365f64;
    if inv_prob < 1f64 - prob {
      return (i, 1f64 - inv_prob);
    }
  }
  unreachable!()
}
